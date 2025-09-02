use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use tokio::process::Command;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use crate::types::*;

// ====== Embedded static assets (built by Vite into ../dist) ======
#[derive(RustEmbed)]
#[folder = "dist"]
struct Assets;

fn embed_file(path: &str) -> Option<(Vec<u8>, HeaderMap)> {
    let asset = Assets::get(path)?;
    let body = asset.data.to_vec();
    let mime = from_path(path).first_or_octet_stream();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, mime.as_ref().parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "public, max-age=3600".parse().unwrap());
    Some((body, headers))
}

async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    // Try exact path first
    if let Some((bytes, headers)) = embed_file(&path) {
        return (headers, bytes).into_response();
    }

    // If path ends with '/', try index.html within it
    if path.ends_with('/') {
        let p = format!("{}index.html", path);
        if let Some((bytes, headers)) = embed_file(&p) {
            return (headers, bytes).into_response();
        }
    }

    // Fallback to root index.html for SPA
    match embed_file("index.html") {
        Some((bytes, headers)) => (headers, bytes).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            "index.html not embedded; run `bun run build` first".to_string(),
        )
            .into_response(),
    }
}

async fn index_handler() -> impl IntoResponse {
    match embed_file("index.html") {
        Some((bytes, headers)) => (headers, bytes).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            "index.html not embedded; run `bun run build` first".to_string(),
        )
            .into_response(),
    }
}

// ====== ZFS logic ======

async fn get_zfs_stats_handler() -> impl IntoResponse {
    match get_zfs_stats().await {
        Ok(stats) => Json(stats).into_response(),
        Err(e) => {
            error!("ZFS error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e).into_response()
        }
    }
}

async fn get_zfs_stats() -> Result<ZfsStats, String> {
    info!("Fetching ZFS stats...");

    let output = Command::new("zfs")
        .args(["list", "-t", "all", "-j"])
        .output()
        .await
        .map_err(|e| format!("Failed to execute zfs command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ZFS command failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let zfs_output: ZfsListOutput = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse ZFS JSON output: {}", e))?;

    let mut pools = Vec::new();
    let mut filesystems = Vec::new();
    let mut snapshots = Vec::new();
    let mut bookmarks = Vec::new();
    let mut total_used_bytes: u64 = 0;
    let mut total_available_bytes: u64 = 0;

    for (_, dataset) in zfs_output.datasets {
        if !pools.contains(&dataset.pool) {
            pools.push(dataset.pool.clone());
        }

        match dataset.dataset_type.as_str() {
            "FILESYSTEM" => {
                if !dataset.name.contains('/') || dataset.name.matches('/').count() == 1 {
                    if let Ok(used) = parse_size_string(&dataset.properties.used.value) {
                        total_used_bytes += used;
                    }
                    if let Ok(available) = parse_size_string(&dataset.properties.available.value) {
                        total_available_bytes += available;
                    }
                }
                filesystems.push(dataset);
            }
            "SNAPSHOT" => snapshots.push(dataset),
            "BOOKMARK" => bookmarks.push(dataset),
            _ => {}
        }
    }

    pools.sort();
    filesystems.sort_by(|a, b| a.name.cmp(&b.name));
    snapshots.sort_by(|a, b| a.name.cmp(&b.name));
    bookmarks.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(ZfsStats {
        pools,
        filesystems,
        snapshots,
        bookmarks,
        total_used: format_bytes(total_used_bytes),
        total_available: format_bytes(total_available_bytes),
    })
}

fn parse_size_string(size_str: &str) -> Result<u64, String> {
    if size_str == "-" || size_str.is_empty() {
        return Ok(0);
    }

    let size_str = size_str.trim();
    if size_str.ends_with('B') {
        if size_str == "0B" {
            return Ok(0);
        }
        let without_b = &size_str[..size_str.len() - 1];
        if let Ok(val) = without_b.parse::<f64>() {
            return Ok(val as u64);
        }
    }

    let (number_part, suffix) = if let Some(last_char) = size_str.chars().last() {
        if last_char.is_alphabetic() {
            (&size_str[..size_str.len() - 1], last_char)
        } else {
            (size_str, ' ')
        }
    } else {
        return Err(format!("Invalid size string: {}", size_str));
    };

    let number: f64 = number_part
        .parse()
        .map_err(|_| format!("Invalid number in size string: {}", size_str))?;

    let multiplier = match suffix {
        'K' => 1024,
        'M' => 1024 * 1024,
        'G' => 1024 * 1024 * 1024,
        'T' => 1024_u64.pow(4),
        'P' => 1024_u64.pow(5),
        'E' => 1024_u64.pow(6),
        ' ' => 1,
        _ => return Err(format!("Unknown size suffix: {}", suffix)),
    };

    Ok((number * multiplier as f64) as u64)
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P", "E"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0B".to_string();
    }

    let bytes_f = bytes as f64;
    let unit_index = (bytes_f.log2() / THRESHOLD.log2()).floor() as usize;
    let unit_index = unit_index.min(UNITS.len() - 1);

    let value = bytes_f / THRESHOLD.powi(unit_index as i32);

    if unit_index == 0 {
        format!("{}B", bytes)
    } else if value >= 100.0 {
        format!("{:.0}{}", value, UNITS[unit_index])
    } else if value >= 10.0 {
        format!("{:.1}{}", value, UNITS[unit_index])
    } else {
        format!("{:.2}{}", value, UNITS[unit_index])
    }
}

#[tokio::main]
async fn main() {
    // Init logging (defaults to info if RUST_LOG not set)
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,tower_http=info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();

    let api = Router::new().route("/zfs", get(get_zfs_stats_handler));

    let app = Router::new()
        .nest("/api", api)
        .route("/", get(index_handler))
        .route("/*path", get(static_handler))
        .layer(TraceLayer::new_for_http());

    let ip: std::net::IpAddr = std::env::var("HOST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| "0.0.0.0".parse().unwrap());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from((ip, port));
    info!("Starting server at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind address");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("install signal handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

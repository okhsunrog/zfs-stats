use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command;

// Simple example command to demonstrate Specta + Tauri integration

#[tauri::command]
#[specta::specta]
pub fn greet(name: String) -> String {
    tracing::info!("Received greet request for {name}");
    format!("Hello, {}! You've been greeted from Rust!", name)
}



// ZFS-related types and commands

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PropertySource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Property {
    pub value: String,
    pub source: PropertySource,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct DatasetProperties {
    pub used: Property,
    pub available: Property,
    pub referenced: Property,
    pub mountpoint: Property,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Dataset {
    pub name: String,
    #[serde(rename = "type")]
    pub dataset_type: String,
    pub pool: String,
    pub createtxg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>, // For snapshots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>, // For snapshots
    pub properties: DatasetProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct OutputVersion {
    pub command: String,
    pub vers_major: u32,
    pub vers_minor: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ZfsListOutput {
    pub output_version: OutputVersion,
    pub datasets: HashMap<String, Dataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ZfsStats {
    pub pools: Vec<String>,
    pub filesystems: Vec<Dataset>,
    pub snapshots: Vec<Dataset>,
    pub bookmarks: Vec<Dataset>,
    pub total_used: String,
    pub total_available: String,
}

#[tauri::command]
#[specta::specta]
pub async fn get_zfs_stats() -> Result<ZfsStats, String> {
    tracing::info!("Fetching ZFS stats...");
    
    // Execute zfs list command
    let output = Command::new("zfs")
        .args(&["list", "-t", "all", "-j"])
        .output()
        .await
        .map_err(|e| format!("Failed to execute zfs command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ZFS command failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse JSON output
    let zfs_output: ZfsListOutput = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse ZFS JSON output: {}", e))?;

    // Process and organize the data
    let mut pools = Vec::new();
    let mut filesystems = Vec::new();
    let mut snapshots = Vec::new();
    let mut bookmarks = Vec::new();
    let mut total_used_bytes: u64 = 0;
    let mut total_available_bytes: u64 = 0;

    for (_, dataset) in zfs_output.datasets {
        // Collect unique pools
        if !pools.contains(&dataset.pool) {
            pools.push(dataset.pool.clone());
        }

        // Categorize datasets
        match dataset.dataset_type.as_str() {
            "FILESYSTEM" => {
                // Only count root filesystems for totals to avoid double-counting
                if !dataset.name.contains('/') || dataset.name.matches('/').count() == 1 {
                    if let Ok(used) = parse_size_string(&dataset.properties.used.value) {
                        total_used_bytes += used;
                    }
                    if let Ok(available) = parse_size_string(&dataset.properties.available.value) {
                        total_available_bytes += available;
                    }
                }
                filesystems.push(dataset);
            },
            "SNAPSHOT" => snapshots.push(dataset),
            "BOOKMARK" => bookmarks.push(dataset),
            _ => {
                tracing::warn!("Unknown dataset type: {}", dataset.dataset_type);
            }
        }
    }

    pools.sort();
    filesystems.sort_by(|a, b| a.name.cmp(&b.name));
    snapshots.sort_by(|a, b| a.name.cmp(&b.name));
    bookmarks.sort_by(|a, b| a.name.cmp(&b.name));

    let stats = ZfsStats {
        pools,
        filesystems,
        snapshots,
        bookmarks,
        total_used: format_bytes(total_used_bytes),
        total_available: format_bytes(total_available_bytes),
    };

    tracing::info!("Successfully fetched ZFS stats: {} pools, {} filesystems, {} snapshots", 
                   stats.pools.len(), stats.filesystems.len(), stats.snapshots.len());

    Ok(stats)
}

// Helper function to parse ZFS size strings to bytes
fn parse_size_string(size_str: &str) -> Result<u64, String> {
    if size_str == "-" || size_str.is_empty() {
        return Ok(0);
    }

    let size_str = size_str.trim();
    if size_str.ends_with('B') {
        if size_str == "0B" {
            return Ok(0);
        }
        // Remove 'B' suffix
        let without_b = &size_str[..size_str.len() - 1];
        if let Ok(val) = without_b.parse::<f64>() {
            return Ok(val as u64);
        }
    }

    // Handle other suffixes
    let (number_part, suffix) = if let Some(last_char) = size_str.chars().last() {
        if last_char.is_alphabetic() {
            (&size_str[..size_str.len() - 1], last_char)
        } else {
            (size_str, ' ')
        }
    } else {
        return Err(format!("Invalid size string: {}", size_str));
    };

    let number: f64 = number_part.parse()
        .map_err(|_| format!("Invalid number in size string: {}", size_str))?;

    let multiplier = match suffix {
        'K' => 1024,
        'M' => 1024 * 1024,
        'G' => 1024 * 1024 * 1024,
        'T' => 1024_u64.pow(4),
        'P' => 1024_u64.pow(5),
        'E' => 1024_u64.pow(6),
        ' ' => 1, // No suffix, assume bytes
        _ => return Err(format!("Unknown size suffix: {}", suffix)),
    };

    Ok((number * multiplier as f64) as u64)
}

// Helper function to format bytes as human-readable string
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

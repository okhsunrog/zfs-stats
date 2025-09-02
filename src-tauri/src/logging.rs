use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tracing_subscriber::{prelude::*, EnvFilter};

// Event structure forwarded to frontend
#[derive(Debug, Clone, Serialize, specta::Type, tauri_specta::Event)]
pub struct LogEvent {
    pub message: String,
}

// Writer that emits logs as events to the frontend
struct TauriWriter {
    app_handle: AppHandle,
}

impl std::io::Write for TauriWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(message) = String::from_utf8(buf.to_vec()) {
            if !message.trim().is_empty() {
                let _ = self.app_handle.emit(
                    "log-event",
                    LogEvent { message },
                );
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

pub fn init_tracing(app_handle: AppHandle) {
    let filter = EnvFilter::from_default_env();

    #[cfg(debug_assertions)]
    let filter = filter
        .add_directive("app_lib=trace".parse().unwrap())
        .add_directive("tauri=info".parse().unwrap())
        .add_directive("debug".parse().unwrap());

    #[cfg(not(debug_assertions))]
    let filter = filter
        .add_directive("app_lib=info".parse().unwrap())
        .add_directive("warn".parse().unwrap());

    let tauri_writer_factory = move || TauriWriter { app_handle: app_handle.clone() };

    #[cfg(target_os = "android")]
    {
        use tracing_logcat::{LogcatMakeWriter, LogcatTag};
        let tag = LogcatTag::Fixed("TauriTemplate".to_owned());
        let logcat_writer = LogcatMakeWriter::new(tag).expect("Failed to init logcat writer");

        let logcat_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(logcat_writer);

        let tauri_layer = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_writer(tauri_writer_factory);

        tracing_subscriber::registry()
            .with(filter)
            .with(logcat_layer)
            .with(tauri_layer)
            .init();
    }

    #[cfg(not(target_os = "android"))]
    {
        let stdout_layer = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_writer(std::io::stdout);

        let tauri_layer = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_writer(tauri_writer_factory);

        tracing_subscriber::registry()
            .with(filter)
            .with(stdout_layer)
            .with(tauri_layer)
            .init();
    }
}


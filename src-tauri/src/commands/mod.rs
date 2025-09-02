// Simple example command to demonstrate Specta + Tauri integration

#[tauri::command]
#[specta::specta]
pub fn greet(name: String) -> String {
    tracing::info!("Received greet request for {name}");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
pub fn emit_test_logs() {
    tracing::trace!("Trace: example trace message");
    tracing::debug!("Debug: example debug message");
    tracing::info!("Info: example info message");
    tracing::warn!("Warn: example warning message");
    tracing::error!("Error: example error message");
}

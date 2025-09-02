pub mod commands;
pub mod logging;
use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Setup Specta builder and register commands/events
    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![commands::greet, commands::emit_test_logs])
        .events(tauri_specta::collect_events![logging::LogEvent]);

    // Export TypeScript bindings in debug mode on desktop
    #[cfg(all(debug_assertions, not(target_os = "android")))]
    {
        let _ = specta_builder.export(
            specta_typescript::Typescript::default()
                .formatter(specta_typescript::formatter::prettier)
                .header("/* eslint-disable */\n// @ts-nocheck"),
            "../src/bindings.ts",
        );
    }

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_log::Builder::new().skip_logger().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(specta_builder.invoke_handler());

    builder
        .setup(move |app| {
            // init tracing/logging and mount events
            logging::init_tracing(app.handle().clone());
            info!("Logging initialized.");
            specta_builder.mount_events(app);
            info!("Specta events mounted.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

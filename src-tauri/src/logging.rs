use tracing_subscriber::{prelude::*, EnvFilter};

pub fn init_tracing() {
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

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_writer(std::io::stdout);

    tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .init();
}
use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the tracing subscriber with JSONL formatting for file output
/// and human-readable formatting for stdout.
pub fn init_telemetry() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("Telemetry initialized");
}

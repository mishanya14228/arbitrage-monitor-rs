use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing() -> Result<(), anyhow::Error> {
    // Initialize logger with colors, timestamps, levels, and function timing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_target(true) // Show module name (context)
        .with_line_number(true) // Show line numbers
        .with_thread_ids(false) // Don't show thread info for cleaner output
        .with_span_events(fmt::format::FmtSpan::CLOSE) // Log function exit with timing
        .init();

    Ok(())
}

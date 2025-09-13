mod adapters;
mod common;
mod monitor;

use crate::common::init_tracing;
use crate::monitor::ArbitrageMonitor;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing().expect("Logger crashed");

    let mut monitor = ArbitrageMonitor::new();
    monitor.start_monitoring().await?;

    Ok(())
}

mod adapters;
mod common;

use crate::adapters::{BinanceAdapter, ExchangeAdapter};
use crate::common::init_tracing;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing();

    info!("🚀 Starting crypto arbitrage monitor");

    let binance = BinanceAdapter::new();
    let markets = binance.get_markets().await?;

    info!(
        exchange = binance.name(),
        market_count = markets.len(),
        "✅ Successfully fetched markets"
    );

    Ok(())
}

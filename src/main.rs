mod adapters;
mod common;

use crate::adapters::common::types::SimpleTicker;
use crate::adapters::{BinanceAdapter, ExchangeAdapter};
use crate::common::init_tracing;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing().expect("Logger crashed");

    info!("🚀 Starting crypto arbitrage monitor");

    let mut binance = BinanceAdapter::new();
    binance.load_markets().await?;

    info!(
        exchange = binance.name(),
        spot_market_count = binance.markets.spot.len(),
        swap_market_count = binance.markets.swap.len(),
        "✅ Successfully fetched markets"
    );

    let tickers = binance.fetch_tickers(None).await?;

    println!("{:#?}", &tickers[..5.min(tickers.len())]);

    Ok(())
}

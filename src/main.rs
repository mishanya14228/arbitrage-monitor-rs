mod adapters;
mod common;
mod monitor;

use crate::common::{init_tracing, ExchangeType};
use crate::monitor::ArbitrageMonitor;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing().expect("Logger crashed");
    let mut monitor = ArbitrageMonitor::new();
    monitor.start_monitoring().await;
    monitor.debug();
    if let Some(adapter) = monitor.get_adapter(&ExchangeType::Binance) {
        
        adapter.fetch_tickers().await?;
        // let tickers = adapter.fetch_spot_tickers(Some(vec!["BTC/USDT"])).await?;
        // let swap_tickers = adapter.fetch_swap_tickers(Some("BTC/USDT:PERP")).await?;
        // println!("{:#?}", &tickers[..2.min(tickers.len())]);
        // println!("{:#?}", &swap_tickers[..2.min(swap_tickers.len())]);
    }
    Ok(())
}

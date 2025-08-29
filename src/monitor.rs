use crate::adapters::ExchangeAdapter;
use crate::common::ExchangeType;
use chrono::Utc;
use cron::Schedule;
use futures::future::join_all;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

pub struct ArbitrageMonitor<const N: usize> {
    exchanges: [ExchangeType; N],
    adapters: HashMap<ExchangeType, Box<dyn ExchangeAdapter>>,
}

impl<const N: usize> fmt::Debug for ArbitrageMonitor<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArbitrageMonitor")
            .field("exchanges", &self.exchanges)
            .field("adapters", &format!("<{} adapters>", self.adapters.len()))
            .finish()
    }
}

impl ArbitrageMonitor<1> {
    pub fn new() -> Self {
        info!("🚀 Initialized crypto arbitrage monitor");
        let exchanges = [ExchangeType::Binance];
        let mut adapters = HashMap::new();
        for &exchange in &exchanges {
            let adapter = exchange.create_adapter();
            adapters.insert(exchange, adapter);
        }

        Self {
            exchanges,
            adapters,
        }
    }

    pub async fn start_monitoring(&mut self) -> Result<(), anyhow::Error> {
        info!("Starting initial ticker fetch...");
        self.refetch_tickers().await?;

        let schedule = Schedule::from_str("*/30 * * * * *")?;
        info!("Starting periodic ticker updates every 30 seconds...");

        loop {
            let now = Utc::now();

            if let Some(next) = schedule.upcoming(Utc).next() {
                let duration_until_next = (next - now).to_std().unwrap_or(Duration::from_secs(30));

                sleep(duration_until_next).await;

                if let Err(e) = self.refetch_tickers().await {
                    error!("Ticker update failed: {}", e);
                } else {
                    self.debug();
                    self.post_fetch()
                }
            }
        }
    }

    pub fn debug(&self) {
        for (_, adapter) in self.adapters.iter() {
            info!(
                exchange = adapter.name(),
                spot_market_count = adapter.get_spot_markets().len(),
                swap_market_count = adapter.get_swap_markets().len(),
                "✅ Fetched tickers"
            );
        }
    }

    // pub fn get_adapter(&self, name: &ExchangeType) -> Option<&dyn ExchangeAdapter> {
    //     self.adapters.get(name).map(|adapter| adapter.as_ref())
    // }

    fn post_fetch(&self) {
        todo!();
    }

    pub async fn refetch_tickers(&mut self) -> Result<(), anyhow::Error> {
        let futures: Vec<_> = self
            .adapters
            .iter_mut()
            .map(|(_, adapter)| adapter.update_tickers())
            .collect();
        let results = join_all(futures).await;
        for result in results {
            result?;
        }
        Ok(())
    }
}

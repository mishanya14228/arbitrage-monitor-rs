use crate::adapters::common::ticker_dataset::TickerDataset;
use crate::adapters::common::ticker_dataset_utils::FilteringConfig;
use crate::adapters::common::types::AdaptersMap;
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
    adapters: AdaptersMap,

    interval_seconds: u32,
}

impl<const N: usize> fmt::Debug for ArbitrageMonitor<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArbitrageMonitor")
            .field("exchanges", &self.exchanges)
            .field("adapters", &format!("<{} adapters>", self.adapters.len()))
            .finish()
    }
}

impl ArbitrageMonitor<2> {
    pub async fn new() -> Self {
        info!("🚀 Initialized crypto arbitrage monitor");
        let exchanges = [
            // ExchangeType::Binance,
            ExchangeType::Bybit,
            ExchangeType::OKX,
            // ExchangeType::Gate,
            // ExchangeType::Bitget,
            // ExchangeType::Kucoin,
        ];
        let mut adapters = HashMap::new();
        for &exchange in &exchanges {
            let adapter = exchange.create_adapter().await;
            adapters.insert(exchange, adapter);
        }

        Self {
            exchanges,
            adapters,
            interval_seconds: 30,
        }
    }

    async fn tick(&mut self) {
        if let Err(e) = self.refetch_tickers().await {
            error!("Ticker update failed: {}", e);
        } else {
            self.debug();
            self.post_fetch()
        }
    }

    pub async fn start_monitoring(&mut self) -> Result<(), anyhow::Error> {
        info!("Starting initial ticker fetch...");
        self.tick().await;

        let schedule = Schedule::from_str(&format!("*/{} * * * * *", self.interval_seconds))?;
        info!(
            interval = self.interval_seconds,
            "Starting periodic ticker updates every"
        );

        loop {
            let now = Utc::now();

            if let Some(next) = schedule.upcoming(Utc).next() {
                let duration_until_next = (next - now).to_std().unwrap_or(Duration::from_secs(30));

                sleep(duration_until_next).await;
                self.tick().await;
            }
        }
    }

    pub fn debug(&self) {
        for (_, adapter) in self.adapters.iter() {
            info!(
                exchange = adapter.name(),
                spot_count = adapter.get_spot_markets().len(),
                swap_count = adapter.get_swap_markets().len(),
                "✅ Fetched tickers"
            );
        }
    }

    fn post_fetch(&self) {
        let initial_dataset: TickerDataset = (&self.adapters).into();
        initial_dataset.debug_df();
        let mut filtered_dataset = initial_dataset.clone();
        filtered_dataset.apply_filters(FilteringConfig::default());
        let spreads = filtered_dataset.calculate_price_spreads();
        match spreads {
            Err(err) => {
                error!("Error calculating price spreads: {}", err);
            }
            Ok(spreads_ldf) => {
                TickerDataset::debug_spread_df(spreads_ldf);
            }
        }
    }

    pub async fn refetch_tickers(&mut self) -> Result<(), anyhow::Error> {
        let futures: Vec<_> = self
            .adapters
            .iter_mut()
            .map(|(_, adapter)| adapter.update_tickers())
            .collect();
        let results = join_all(futures).await;
        for result in results {
            if let Err(e) = result {
                error!("Error while refetching tickers: {}", e);
            }
        }
        Ok(())
    }
}

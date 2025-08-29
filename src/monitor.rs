use crate::adapters::ExchangeAdapter;
use crate::common::ExchangeType;
use futures::future::join_all;
use std::collections::HashMap;
use std::fmt;
use tracing::info;

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

    pub async fn start_monitoring(&mut self) {
        // let futures: Vec<_> = self
        //     .adapters
        //     .iter_mut()
        //     .map(|(_, adapter)| adapter.load_markets())
        //     .collect();
        // join_all(futures).await;
    }

    pub fn debug(&self) {
        for (_, adapter) in self.adapters.iter() {
            info!(
                exchange = adapter.name(),
                spot_market_count = adapter.get_spot_markets().len(),
                swap_market_count = adapter.get_swap_markets().len(),
                "✅ Successfully fetched markets"
            );
            //     let tickers = adapter.fetch_tickers(None).await?;
            //     println!("{:#?}", &tickers[..2.min(tickers.len())]);
            //     println!("{:#?}", &tickers[tickers.len().saturating_sub(2)..]);
        }
    }

    pub fn get_adapter(&self, name: &ExchangeType) -> Option<&dyn ExchangeAdapter> {
        self.adapters.get(name).map(|adapter| adapter.as_ref())
    }
}

use crate::adapters::common::types::{AdaptersMap, Ticker24HrChange};
use crate::common::ExchangeType;
use std::collections::HashMap;
use tracing::info;

pub struct TickerDataset {
    dataset: HashMap<ExchangeType, Vec<Ticker24HrChange>>,
}

impl TickerDataset {
    pub fn debug(&self) {
        let mut markets: HashMap<ExchangeType, Vec<Ticker24HrChange>> = HashMap::new();
        self.dataset.iter().for_each(|(&exchange_type, tickers)| {
            markets.entry(exchange_type).or_insert(tickers[..2].to_vec());
        });
        info!("Tickers Dataset: {:#?}", markets);
    }
}

impl From<&AdaptersMap> for TickerDataset {
    fn from(adapters: &AdaptersMap) -> Self {
        let mut markets: HashMap<ExchangeType, Vec<Ticker24HrChange>> = HashMap::new();

        adapters.iter().for_each(|(&name, adapter)| {
            markets
                .entry(name)
                .or_insert(adapter.get_swap_markets());
        });

        TickerDataset { dataset: markets }
    }
}

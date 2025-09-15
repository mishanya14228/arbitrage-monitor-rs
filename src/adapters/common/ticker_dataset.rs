use super::ticker_dataset_utils::{BlacklistedTicker, FilteringConfig};
use super::types::{AdaptersMap, Ticker24HrChange};
use crate::common::ExchangeType;
use polars::df;
use polars::prelude::*;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/** DATASET CODE */
type TickersHashMap = HashMap<ExchangeType, Vec<Ticker24HrChange>>;

pub struct TickerDataset {
    dataset: TickersHashMap,
    dataframe: LazyFrame,
}

impl TickerDataset {
    pub fn debug(&self) {
        let frame_data = self.dataframe.clone().collect();
        // let frame_data = self
        //     .apply_filters(FilteringConfig::new(
        //         20.0,
        //         20,
        //         vec![
        //             BlacklistedTicker {
        //                 name: String::from("ALPACA/USDT:PERP"),
        //                 exchange: ExchangeType::Binance,
        //             },
        //             BlacklistedTicker {
        //                 name: String::from("AVNT/USDT:PERP"),
        //                 exchange: ExchangeType::Binance,
        //             },
        //             BlacklistedTicker {
        //                 name: String::from("AVNT/USDT:PERP"),
        //                 exchange: ExchangeType::Gate,
        //             },
        //         ],
        //     ))
        //     .collect();
        info!("Tickers Dataset: {:#?}", frame_data);
    }

    fn create_df(dataset: TickersHashMap) -> LazyFrame {
        let mut exchanges = Vec::new();
        let mut symbols = Vec::new();
        let mut unified_symbols = Vec::new();
        let mut percentage_changes = Vec::new();

        for (exchange, tickers) in dataset {
            for ticker in tickers {
                exchanges.push(format!("{:?}", exchange));
                symbols.push(ticker.symbol);
                unified_symbols.push(ticker.unified_symbol);
                percentage_changes.push(ticker.percentage_change);
            }
        }
        df![
            "exchange" => exchanges,
            "symbol" => symbols,
            "unified_symbol" => unified_symbols,
            "percentage_change" => percentage_changes
        ]
        .expect("Valid dataframe")
        .lazy()
    }

    pub fn apply_filters(&self, filters: FilteringConfig) -> LazyFrame {
        let mut filtered_df = self.dataframe.clone();

        filtered_df = filtered_df.join(
            filters.blacklist.into(),
            [col("exchange"), col("unified_symbol")],
            [col("exchange"), col("unified_symbol")],
            JoinArgs::new(JoinType::Anti),
        );
        //
        // let quantile_val = filtered_df.clone()
        //     .select([col("percentage_change").quantile(lit(filters.quantile), QuantileMethod::Nearest)])
        //     .collect()
        //     .unwrap()
        //     .column("percentage_change")
        //     .unwrap()
        //     .get(0)
        //     .unwrap(); // get the scalar value
        //
        // let quantile_threshold = match quantile_val {
        //     AnyValue::Float64(v) => v,
        //     _ => {
        //         warn!("Couldnt parse threshold, returning 25 as default");
        //         25f64
        //     }
        // };

        // println!("Quantiles: {:?}", quantile_threshold);
        // 
        // filtered_df = filtered_df.filter(col("percentage_change").gt_eq(lit(quantile_threshold)));

        filtered_df
    }
}

impl From<&AdaptersMap> for TickerDataset {
    fn from(adapters: &AdaptersMap) -> Self {
        let mut markets: HashMap<ExchangeType, Vec<Ticker24HrChange>> = HashMap::new();

        adapters.iter().for_each(|(&name, adapter)| {
            markets.entry(name).or_insert(adapter.get_swap_markets());
        });

        let dataframe = TickerDataset::create_df(markets.clone())
            .sort(["percentage_change"], SortMultipleOptions::default());

        TickerDataset {
            dataset: markets,
            dataframe,
        }
    }
}

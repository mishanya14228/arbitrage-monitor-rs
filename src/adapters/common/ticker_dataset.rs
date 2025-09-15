use super::ticker_dataset_utils::FilteringConfig;
use super::types::{AdaptersMap, Ticker24HrChange};
use crate::common::ExchangeType;
use polars::df;
use polars::prelude::*;
use std::collections::HashMap;
use tracing::{debug, warn};

/** DATASET CODE */
type TickersHashMap = HashMap<ExchangeType, Vec<Ticker24HrChange>>;

#[derive(Clone)]
pub struct TickerDataset {
    dataset: TickersHashMap,
    dataframe: LazyFrame,
    filters: Option<FilteringConfig>,
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
            filters: None,
        }
    }
}

impl TickerDataset {
    fn create_df(dataset: TickersHashMap) -> LazyFrame {
        let mut exchanges = Vec::new();
        let mut symbols = Vec::new();
        let mut unified_symbols = Vec::new();
        let mut percentage_changes = Vec::new();
        let mut last_prices = Vec::new();
        let mut volumes = Vec::new();

        for (exchange, tickers) in dataset {
            for ticker in tickers {
                exchanges.push(format!("{:?}", exchange));
                symbols.push(ticker.symbol);
                unified_symbols.push(ticker.unified_symbol);
                percentage_changes.push(ticker.percentage_change);
                last_prices.push(ticker.last_price);
                volumes.push(ticker.volume_usd)
            }
        }

        df![
            "exchange" => exchanges,
            "symbol" => symbols,
            "unified_symbol" => unified_symbols,
            "percentage_change" => percentage_changes,
            "last_price" => last_prices,
            "volume" => volumes
        ]
        .expect("Valid dataframe")
        .lazy()
    }

    pub fn apply_filters(&mut self, filters: FilteringConfig) {
        self.filters = Option::from(filters.clone());
        let ldf = self.dataframe.clone();
        self.dataframe = ldf
            .join(
                filters.blacklist.into(),
                [col("exchange"), col("unified_symbol")],
                [col("exchange"), col("unified_symbol")],
                JoinArgs::new(JoinType::Anti),
            )
            .filter(col("volume").gt_eq(filters.volume_threshold));
    }

    pub fn calculate_price_spreads(&mut self) -> PolarsResult<DataFrame> {
        let df = self.dataframe.clone();
        let spreads = df
            .clone()
            .join(
                df,
                [col("unified_symbol")],
                [col("unified_symbol")],
                JoinArgs::new(JoinType::Inner),
            )
            // Filter out self-comparisons (exchange == exchange_right)
            .filter(col("exchange").neq(col("exchange_right")))
            // Calculate absolute and percentage spreads
            .with_columns([
                (col("last_price") - col("last_price_right")).alias("absolute_spread"),
                (((col("last_price") - col("last_price_right")) / col("last_price_right"))
                    * lit(100.0))
                .alias("percentage_spread"),
            ])
            .sort(
                ["percentage_spread"],
                SortMultipleOptions::default().with_order_descending(true),
            );
        if let Some(filters) = &self.filters {
            spreads
                .filter(col("percentage_spread").gt_eq(filters.spread_threshold))
                .collect()
        } else {
            spreads.collect()
        }
    }

    pub fn debug_df(&self) {
        let collected = self.dataframe.clone().collect();
        match collected {
            Ok(df) => {
                debug!("TickerDataset: {:?}", df)
            }
            Err(err) => {
                warn!("TickerDataset::debug_df returned an error: {err}");
            }
        }
    }

    pub fn debug_spread_df(ldf: DataFrame) {
        let spreads_result = ldf
            .lazy()
            .with_columns([when(col("volume").lt(col("volume_right")))
                .then(col("volume"))
                .otherwise(col("volume_right"))
                .alias("min_volume")])
            .select([
                col("exchange").alias("short"),
                col("exchange_right").alias("long"),
                col("unified_symbol").alias("symbol"),
                col("last_price").alias("price"),
                col("min_volume"),
                // col("absolute_spread"),
                col("percentage_spread").alias("spread"),
            ])
            .collect();
        match spreads_result {
            Ok(_) => {
                println!("Ticker dataset spreads: {:#?}", spreads_result);
            }
            Err(_) => {
                warn!("Ticker dataset spreads returned an error");
            }
        }
    }
}

use super::ticker_dataset_utils::FilteringConfig;
use super::types::{AdaptersMap, TickerBidAsk};
use crate::common::ExchangeType;
use polars::df;
use polars::prelude::*;
use std::collections::HashMap;
use tracing::{debug, warn};

/** DATASET CODE */
type TickersHashMap = HashMap<ExchangeType, Vec<TickerBidAsk>>;

#[derive(Clone)]
pub struct TickerDataset {
    dataset: TickersHashMap,
    dataframe: LazyFrame,
    filters: Option<FilteringConfig>,
}

impl From<&AdaptersMap> for TickerDataset {
    fn from(adapters: &AdaptersMap) -> Self {
        let mut markets: HashMap<ExchangeType, Vec<TickerBidAsk>> = HashMap::new();

        adapters.iter().for_each(|(&name, adapter)| {
            markets.entry(name).or_insert(adapter.get_swap_markets());
        });

        let dataframe = TickerDataset::create_df(markets.clone())
            .sort(["unified_symbol"], SortMultipleOptions::default());

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
        let mut bids = Vec::new();
        let mut asks = Vec::new();
        let mut volumes = Vec::new();

        for (exchange, tickers) in dataset {
            for ticker in tickers {
                exchanges.push(format!("{:?}", exchange));
                symbols.push(ticker.symbol);
                unified_symbols.push(ticker.unified_symbol);
                bids.push(ticker.bid);
                asks.push(ticker.ask);
                volumes.push(ticker.volume_usd)
            }
        }

        df![
            "exchange" => exchanges,
            "symbol" => symbols,
            "unified_symbol" => unified_symbols,
            "bid" => bids,
            "ask" => asks,
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
                JoinArgs::new(JoinType::Inner).with_suffix(Some(PlSmallStr::from_str("_long"))),
            )
            // Filter out self-comparisons (exchange == exchange_long)
            .filter(col("exchange").neq(col("exchange_long")))
            /* get avg prices */
            .with_columns([
                ((col("bid") + col("ask_long")) / lit(2)).alias("open_avg_price"),
                ((col("bid_long") + col("ask")) / lit(2)).alias("exit_avg_price"),
            ])
            /* get open/close diffs */
            .with_columns([
                (col("bid") - col("ask_long")).alias("open_diff"),
                (col("bid_long") - col("ask")).alias("exit_diff"),
            ])
            /* calculate spreads */
            .with_columns([
                (col("open_diff") / col("open_avg_price") * lit(100)).alias("entry_spread"),
                (col("exit_diff") / col("exit_avg_price") * lit(100)).alias("exit_spread"),
            ])
            .sort_by_exprs(
                [col("entry_spread")],
                SortMultipleOptions::default().with_order_descending(true),
            );
        if let Some(filters) = &self.filters {
            spreads
                .filter(col("entry_spread").gt_eq(filters.spread_threshold))
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
            .with_columns([when(col("volume").lt(col("volume_long")))
                .then(col("volume"))
                .otherwise(col("volume_long"))
                .alias("min_volume")])
            .select([
                col("exchange").alias("short"),
                col("exchange_long").alias("long"),
                col("unified_symbol").alias("symbol"),
                col("bid").alias("price"),
                col("min_volume"),
                col("entry_spread"),
                col("exit_spread"),
            ])
            .collect();
        match spreads_result {
            Ok(_) => {
                println!("Ticker dataset spreads: {:#?}", spreads_result);
            }
            Err(err) => {
                warn!("Ticker dataset spreads returned an error: {err}");
            }
        }
    }
}

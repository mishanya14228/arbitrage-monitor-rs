use crate::common::ExchangeType;
use polars::df;
use polars::prelude::{IntoLazy, LazyFrame};

/** BLACKLIST CONFIG */
pub struct BlacklistedTicker {
    pub exchange: ExchangeType,
    pub name: String,
}

pub struct BlacklistDataFrame(Vec<BlacklistedTicker>);

impl From<BlacklistDataFrame> for LazyFrame {
    fn from(blacklist: BlacklistDataFrame) -> Self {
        let (exchanges, names): (Vec<_>, Vec<_>) = blacklist
            .0
            .iter()
            .map(|b| (format!("{:?}", b.exchange), b.name.clone()))
            .unzip();

        df![
            "exchange" => exchanges,
            "unified_symbol" => names
        ]
        .unwrap()
        .lazy()
    }
}

/** FILTERING CONFIG */

pub struct FilteringConfig {
    pub quantile: f64,
    pub error_threshold: i32,
    pub blacklist: BlacklistDataFrame,
}

impl FilteringConfig {
    pub fn new(quantile: f64, error_threshold: i32, blacklist: Vec<BlacklistedTicker>) -> Self {
        Self {
            quantile,
            error_threshold,
            blacklist: BlacklistDataFrame(blacklist),
        }
    }
}

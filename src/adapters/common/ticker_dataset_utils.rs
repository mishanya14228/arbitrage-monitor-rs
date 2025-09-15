use crate::common::ExchangeType;
use polars::df;
use polars::prelude::{IntoLazy, LazyFrame};

/** BLACKLIST CONFIG */
#[derive(Clone)]
pub struct BlacklistedTicker {
    pub exchange: ExchangeType,
    pub name: String,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FilteringConfig {
    pub volume_threshold: f64,
    pub spread_threshold: f32,
    pub blacklist: BlacklistDataFrame,
}

impl FilteringConfig {
    pub fn new(
        volume_threshold: f64,
        spread_threshold: f32,
        blacklist: Vec<BlacklistedTicker>,
    ) -> Self {
        Self {
            volume_threshold,
            spread_threshold,
            blacklist: BlacklistDataFrame(blacklist),
        }
    }
}

impl Default for FilteringConfig {
    fn default() -> Self {
        let binance_trash: Vec<BlacklistedTicker> = [
            "ALPACA", "LOOM", "XEM", "GLMR", "RAD", "MDT", "DGB", "IDEX", "ORBS", "BAL", "SNT",
            "MEMEFI", "LEVER", "WAVES", "BADGER", "BSW",
        ]
        .iter()
        .map(|token| BlacklistedTicker {
            name: format!("{}/USDT:PERP", token),
            exchange: ExchangeType::Binance,
        })
        .collect();

        Self::new(0_500_000.0, 0.1, binance_trash)
    }
}

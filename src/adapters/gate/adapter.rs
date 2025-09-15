use crate::adapters::common::types::{
    ExchangeApiConfig, ExchangeMarketsInfo, MarketType, Ticker24HrChange,
};
use crate::adapters::gate::types::{GateSpotTickersListDto, GateSwapTickersListDto};
use crate::adapters::ExchangeAdapter;
use anyhow::Error;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use tokio::time::Duration;
use tracing::debug;

pub struct GateAdapter {
    markets: ExchangeMarketsInfo,
    api: ExchangeApiConfig,
}

impl GateAdapter {
    pub fn new() -> GateAdapter {
        Self {
            api: {
                ExchangeApiConfig {
                    spot_url: "https://api.gateio.ws".to_string(),
                    swap_url: "https://api.gateio.ws".to_string(),
                }
            },
            markets: {
                ExchangeMarketsInfo {
                    spot: Vec::new(),
                    swap: Vec::new(),
                }
            },
        }
    }

    async fn fetch_tickers_unified_http_only<T>(&self, endpoint: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}{}", self.api.spot_url, endpoint).as_str())
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

#[async_trait]
impl ExchangeAdapter for GateAdapter {
    fn markets(&self) -> &ExchangeMarketsInfo {
        &self.markets
    }

    fn markets_mut(&mut self) -> &mut ExchangeMarketsInfo {
        &mut self.markets
    }

    fn name(&self) -> &str {
        "Gate"
    }

    fn wrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        match market_type {
            MarketType::Swap => {
                format!("{}_USDT", symbol.strip_suffix("/USDT:PERP").unwrap_or(""))
            }
            MarketType::Spot => {
                format!("{}_USDT", symbol.strip_suffix("/USDT").unwrap_or(""))
            }
        }
    }

    fn unwrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        let base = symbol.strip_suffix("_USDT").unwrap_or("");
        match market_type {
            MarketType::Swap => {
                format!("{}/USDT:PERP", base)
            }
            MarketType::Spot => {
                format!("{}/USDT", base)
            }
        }
    }

    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<&str>>,
    ) -> Result<Vec<Ticker24HrChange>, Error> {
        if let Some(_tickers) = tickers {
            debug!("Gate API doesn't support tickers param yet");
        }
        const ENDPOINT: &str = "/api/v4/spot/tickers";
        let response: GateSpotTickersListDto =
            self.fetch_tickers_unified_http_only(ENDPOINT).await?;
        let tickers = response
            .iter()
            .filter(|ticker| ticker.symbol.ends_with("_USDT"))
            .map(|book_ticker| Ticker24HrChange {
                unified_symbol: self.unwrap_symbol(book_ticker.symbol.clone(), MarketType::Spot),
                symbol: book_ticker.symbol.clone(),
                percentage_change: book_ticker.percentage_change.parse().unwrap_or(0.0),
                last_price: book_ticker.last_price.parse().unwrap_or(0.0),
                volume_usd: book_ticker.volume.parse().unwrap_or(0.0),
            })
            .collect();
        Ok(tickers)
    }

    async fn fetch_swap_tickers(
        &self,
        ticker: Option<&str>,
    ) -> Result<Vec<Ticker24HrChange>, Error> {
        if let Some(_tickers) = ticker {
            debug!("Gate API doesn't support tickers param yet");
        }

        const ENDPOINT: &str = "/api/v4/futures/usdt/tickers";
        let response: GateSwapTickersListDto =
            self.fetch_tickers_unified_http_only(ENDPOINT).await?;
        let tickers = response
            .iter()
            .filter(|ticker| ticker.symbol.ends_with("_USDT"))
            .map(|book_ticker| Ticker24HrChange {
                unified_symbol: self.unwrap_symbol(book_ticker.symbol.clone(), MarketType::Swap),
                symbol: book_ticker.symbol.clone(),
                percentage_change: book_ticker.percentage_change.parse().unwrap_or(0.0),
                last_price: book_ticker.last_price.parse().unwrap_or(0.0),
                volume_usd: book_ticker.volume.parse().unwrap_or(0.0),
            })
            .collect();
        Ok(tickers)
    }
}

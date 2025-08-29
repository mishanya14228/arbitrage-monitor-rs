use crate::adapters::bybit::types::BybitTickersListDto;
use crate::adapters::common::types::{
    ExchangeApiConfig, ExchangeMarketsInfo, MarketType, Ticker24HrChange,
};
use crate::adapters::ExchangeAdapter;
use crate::common::parse_json_response;
use anyhow::Error;
use async_trait::async_trait;
use tracing::debug;

pub struct BybitAdapter {
    markets: ExchangeMarketsInfo,
    api: ExchangeApiConfig,
}

impl BybitAdapter {
    pub fn new() -> Self {
        BybitAdapter {
            api: {
                ExchangeApiConfig {
                    spot_url: "https://api.bybit.com".to_string(),
                    swap_url: "https://api.bybit.com".to_string(),
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

    async fn fetch_tickers_unified(
        &self,
        market_type: MarketType,
    ) -> Result<Vec<Ticker24HrChange>, Error> {
        const ENDPOINT: &str = "/v5/market/tickers";
        let client = reqwest::Client::new();
        let url: &str = &format!("{}{}", self.api.spot_url, ENDPOINT);
        let mut request = client.get(url);
        match market_type {
            MarketType::Spot => {
                request = request.query(&[("category", "spot")]);
            }
            MarketType::Swap => {
                request = request.query(&[("category", "linear")]);
            }
        }

        let raw_response = request.send().await?;
        let response: BybitTickersListDto = parse_json_response(raw_response).await?;

        let tickers = response
            .result
            .list
            .iter()
            .filter(|book_ticker| book_ticker.symbol.ends_with("USDT"))
            .map(|book_ticker| Ticker24HrChange {
                unified_symbol: self.unwrap_symbol(book_ticker.symbol.clone(), market_type),
                symbol: book_ticker.symbol.clone(),
                percentage_change: book_ticker.percentage_change.parse().unwrap_or(0.0),
            })
            .collect();

        Ok(tickers)
    }
}

#[async_trait]
impl ExchangeAdapter for BybitAdapter {
    fn name(&self) -> &str {
        "Bybit"
    }

    fn markets(&self) -> &ExchangeMarketsInfo {
        &self.markets
    }

    fn markets_mut(&mut self) -> &mut ExchangeMarketsInfo {
        &mut self.markets
    }

    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<&str>>,
    ) -> Result<Vec<Ticker24HrChange>, Error> {
        if let Some(tickers) = tickers {
            debug!("Bybit API doesn't support tickers param yet");
        }
        Ok(self.fetch_tickers_unified(MarketType::Spot).await?)
    }

    async fn fetch_swap_tickers(
        &self,
        tickers: Option<&str>,
    ) -> Result<Vec<Ticker24HrChange>, Error> {
        if let Some(tickers) = tickers {
            debug!("Bybit API doesn't support tickers param yet");
        }
        Ok(self.fetch_tickers_unified(MarketType::Swap).await?)
    }
}

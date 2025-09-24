use crate::adapters::common::types::{
    ExchangeApiConfig, ExchangeMarketsInfo, MarketType, TickerBidAsk,
};
use crate::adapters::okx::types::OkxTickersListDto;
use crate::adapters::ExchangeAdapter;
use crate::common::parse_json_response;
use anyhow::Error;
use async_trait::async_trait;
use tracing::debug;

pub struct OkxAdapter {
    markets: ExchangeMarketsInfo,
    api: ExchangeApiConfig,
}

impl OkxAdapter {
    pub fn new() -> Self {
        OkxAdapter {
            api: {
                ExchangeApiConfig {
                    spot_url: "https://www.okx.com".to_string(),
                    swap_url: "https://www.okx.com".to_string(),
                    ws_swap_url: "wss://ws.okx.com:8443/ws/v5/public".to_string(),
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
    ) -> Result<Vec<TickerBidAsk>, Error> {
        const ENDPOINT: &str = "/api/v5/market/tickers";
        let client = reqwest::Client::new();
        let url: &str = &format!("{}{}", self.api.spot_url, ENDPOINT);
        let mut request = client.get(url);
        match market_type {
            MarketType::Spot => {
                request = request.query(&[("instType", "SPOT")]);
            }
            MarketType::Swap => {
                request = request.query(&[("instType", "SWAP")]);
            }
        }
        let raw_response = request.send().await?;
        let response: OkxTickersListDto = parse_json_response(raw_response).await?;
        let tickers = response
            .data
            .iter()
            .filter(|ticker| {
                ticker.symbol.ends_with("USDT") || ticker.symbol.ends_with("USDT-SWAP")
            })
            .map(|ticker| {
                let mut formated_ticker: TickerBidAsk = ticker.into();
                formated_ticker.unified_symbol =
                    self.unwrap_symbol(formated_ticker.unified_symbol, market_type);
                return formated_ticker;
            })
            .collect();
        Ok(tickers)
    }
}

#[async_trait]
impl ExchangeAdapter for OkxAdapter {
    fn markets(&self) -> &ExchangeMarketsInfo {
        &self.markets
    }

    fn markets_mut(&mut self) -> &mut ExchangeMarketsInfo {
        &mut self.markets
    }

    fn name(&self) -> &str {
        "Okx"
    }

    fn wrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        match market_type {
            MarketType::Swap => {
                format!(
                    "{}-USDT-SWAP",
                    symbol.strip_suffix("/USDT:PERP").unwrap_or("")
                )
            }
            MarketType::Spot => {
                format!("{}-USDT", symbol.strip_suffix("/USDT").unwrap_or(""))
            }
        }
    }

    fn unwrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        match market_type {
            MarketType::Swap => {
                let base = symbol.strip_suffix("-USDT-SWAP").unwrap_or("");
                format!("{}/USDT:PERP", base)
            }
            MarketType::Spot => {
                let base = symbol.strip_suffix("-USDT").unwrap_or("");
                format!("{}/USDT", base)
            }
        }
    }

    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<&str>>,
    ) -> Result<Vec<TickerBidAsk>, Error> {
        if let Some(_tickers) = tickers {
            debug!("Okx API doesn't support tickers param yet");
        }
        Ok(self.fetch_tickers_unified(MarketType::Spot).await?)
    }
    async fn fetch_swap_tickers(&self, tickers: Option<&str>) -> Result<Vec<TickerBidAsk>, Error> {
        if let Some(_tickers) = tickers {
            debug!("Okx API doesn't support tickers param yet");
        }
        Ok(self.fetch_tickers_unified(MarketType::Swap).await?)
    }
}

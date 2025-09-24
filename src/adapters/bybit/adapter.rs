use crate::adapters::bybit::types::BybitTickersListDto;
use crate::adapters::common::types::{
    ExchangeApiConfig, ExchangeMarketsInfo, MarketType, TickerBidAsk,
};
use crate::adapters::common::websockets::WebSocketManager;
use crate::adapters::ExchangeAdapter;
use crate::common::parse_json_response;
use anyhow::Error;
use async_trait::async_trait;
use tracing::debug;

pub struct BybitAdapter {
    markets: ExchangeMarketsInfo,
    api: ExchangeApiConfig,
    ws_manager: WebSocketManager,
}

impl BybitAdapter {
    pub fn new() -> Self {
        BybitAdapter {
            api: {
                ExchangeApiConfig {
                    spot_url: "https://api.bybit.com".to_string(),
                    swap_url: "https://api.bybit.com".to_string(),
                    ws_swap_url: "wss://stream.bybit.com/v5/public/linear".to_string(),
                }
            },
            markets: {
                ExchangeMarketsInfo {
                    spot: Vec::new(),
                    swap: Vec::new(),
                }
            },
            ws_manager: WebSocketManager::new(),
        }
    }

    async fn fetch_tickers_unified(
        &self,
        market_type: MarketType,
    ) -> Result<Vec<TickerBidAsk>, Error> {
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
            .map(|book_ticker| {
                let mut ticker: TickerBidAsk = book_ticker.into();
                ticker.set_unified_symbol(
                    self.unwrap_symbol(book_ticker.symbol.clone(), market_type),
                );
                ticker
            })
            .collect();

        Ok(tickers)
    }
}

#[async_trait]
impl ExchangeAdapter for BybitAdapter {
    fn markets(&self) -> &ExchangeMarketsInfo {
        &self.markets
    }

    fn markets_mut(&mut self) -> &mut ExchangeMarketsInfo {
        &mut self.markets
    }

    fn name(&self) -> &str {
        "Bybit"
    }

    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<&str>>,
    ) -> Result<Vec<TickerBidAsk>, Error> {
        if let Some(_tickers) = tickers {
            debug!("Bybit API doesn't support tickers param yet");
        }
        Ok(self.fetch_tickers_unified(MarketType::Spot).await?)
    }

    async fn fetch_swap_tickers(&self, tickers: Option<&str>) -> Result<Vec<TickerBidAsk>, Error> {
        if let Some(_tickers) = tickers {
            debug!("Bybit API doesn't support tickers param yet");
        }
        Ok(self.fetch_tickers_unified(MarketType::Swap).await?)
    }

    async fn init(&self) -> Result<(), Error> {
        // self.ws_manager.connect_all()
        Ok(())
    }

    fn orderbook_subscribe(&self, symbols: Vec<String>) {}
}

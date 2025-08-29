use super::types::BinanceBookTickerDto;
use crate::adapters::common::{
    traits::ExchangeAdapter,
    types::{ExchangeApiConfig, ExchangeMarketsInfo, MarketType, Ticker24HrChange},
};
use crate::common::parse_json_response;
use anyhow::Error;
use async_trait::async_trait;
use tokio;

#[derive(Debug)]
pub struct BinanceAdapter {
    markets: ExchangeMarketsInfo,
    api: ExchangeApiConfig,
}

impl BinanceAdapter {
    pub fn new() -> Self {
        BinanceAdapter {
            api: {
                ExchangeApiConfig {
                    spot_url: "https://api.binance.com".to_string(),
                    swap_url: "https://fapi.binance.com".to_string(),
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

    pub fn map_tickers(
        &self,
        response: Vec<BinanceBookTickerDto>,
        market_type: MarketType,
    ) -> Vec<Ticker24HrChange> {
        response
            .iter()
            .filter(|book_ticker| book_ticker.symbol.ends_with("USDT"))
            .map(|book_ticker| Ticker24HrChange {
                unified_symbol: self.unwrap_symbol(book_ticker.symbol.clone(), market_type),
                symbol: book_ticker.symbol.clone(),
                percentage_change: book_ticker.percentage_change.parse().unwrap_or(0.0),
            })
            .collect()
    }
}

#[async_trait]
impl ExchangeAdapter for BinanceAdapter {
    fn name(&self) -> &str {
        "Binance"
    }

    fn markets(&self) -> &ExchangeMarketsInfo {
        &self.markets
    }

    fn markets_mut(&mut self) -> &mut ExchangeMarketsInfo {
        &mut self.markets
    }

    // #[instrument(level = "info", skip(self))]
    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<&str>>,
    ) -> Result<Vec<Ticker24HrChange>, Error> {
        const ENDPOINT: &str = "/api/v3/ticker/24hr";
        let client = reqwest::Client::new();
        let url: &str = &format!("{}{}", self.api.spot_url, ENDPOINT);
        let mut request = client.get(url);
        if let Some(symbols) = tickers {
            let wrapped_symbols: Vec<String> = symbols
                .iter()
                .map(|x| self.wrap_symbol(x.to_string(), MarketType::Spot))
                .collect();
            let symbols_json = serde_json::to_string(&wrapped_symbols)?;
            request = request.query(&[("symbols", symbols_json)]);
        }
        let raw_response = request.send().await?;
        let response: Vec<BinanceBookTickerDto> = parse_json_response(raw_response).await?;
        let tickers = self.map_tickers(response, MarketType::Spot);
        Ok(tickers)
    }

    // #[instrument(level = "info", skip(self))]
    async fn fetch_swap_tickers(
        &self,
        ticker: Option<&str>,
    ) -> Result<Vec<Ticker24HrChange>, Error> {
        const ENDPOINT: &str = "/fapi/v1/ticker/24hr";
        let client = reqwest::Client::new();
        let url: &str = &format!("{}{}", self.api.swap_url, ENDPOINT);
        let mut request = client.get(url);
        if let Some(symbol) = ticker {
            request = request.query(&[(
                "symbol",
                self.wrap_symbol(symbol.to_string(), MarketType::Swap),
            )]);
        }
        let response: Vec<BinanceBookTickerDto> = if ticker.is_some() {
            let single_ticker: BinanceBookTickerDto =
                parse_json_response(request.send().await?).await?;
            vec![single_ticker]
        } else {
            parse_json_response(request.send().await?).await?
        };
        let tickers = self.map_tickers(response, MarketType::Swap);
        Ok(tickers)
    }
}

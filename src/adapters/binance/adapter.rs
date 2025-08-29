use super::types::BookTickerDto;
use crate::adapters::common::{
    traits::ExchangeAdapter,
    types::{ExchangeApiConfig, ExchangeMarketsInfo, MarketType, Ticker24HrChange},
};
use crate::common::parse_json_response;
use anyhow::Error;
use async_trait::async_trait;
use tokio;
use tracing::instrument;

// Just data - no methods yet
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
        response: Vec<BookTickerDto>,
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

    fn wrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        match market_type {
            MarketType::Swap => {
                format!("{}USDT", symbol.strip_suffix("/USDT:PERP").unwrap_or(""))
            }
            MarketType::Spot => {
                format!("{}USDT", symbol.strip_suffix("/USDT").unwrap_or(""))
            }
        }
    }

    fn unwrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        let base = symbol.strip_suffix("USDT").unwrap_or("");
        match market_type {
            MarketType::Swap => {
                format!("{}/USDT:PERP", base)
            }
            MarketType::Spot => {
                format!("{}/USDT", base)
            }
        }
    }

    fn get_spot_markets(&self) -> Vec<Ticker24HrChange> {
        self.markets.spot.clone()
    }

    fn get_swap_markets(&self) -> Vec<Ticker24HrChange> {
        self.markets.swap.clone()
    }

    #[instrument(level = "info", skip(self))]
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
        let response: Vec<BookTickerDto> = parse_json_response(raw_response).await?;
        let tickers = self.map_tickers(response, MarketType::Spot);
        Ok(tickers)
    }

    #[instrument(level = "info", skip(self))]
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
        let response: Vec<BookTickerDto> = if ticker.is_some() {
            let single_ticker: BookTickerDto = parse_json_response(request.send().await?).await?;
            vec![single_ticker]
        } else {
            parse_json_response(request.send().await?).await?
        };
        let tickers = self.map_tickers(response, MarketType::Swap);
        Ok(tickers)
    }

    async fn fetch_tickers(&self) -> Result<Vec<Ticker24HrChange>, Error> {
        let (spot_tickers, swap_tickers) =
            tokio::join!(self.fetch_spot_tickers(None), self.fetch_swap_tickers(None));

        // Return combined markets
        let mut all_tickers: Vec<Ticker24HrChange> = spot_tickers?.clone();
        all_tickers.extend(swap_tickers?);
        Ok(all_tickers)
    }
}

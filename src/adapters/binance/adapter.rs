use super::types::{BookTickerDto, ExchangeInfoDto, SymbolDto};
use crate::adapters::common::{
    traits::ExchangeAdapter,
    types::{ExchangeApiConfig, ExchangeMarketsInfo, Market, MarketType, Ticker24HrChange},
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

    async fn fetch_and_map_markets<F>(
        &self,
        url: &str,
        endpoint: &str,
        market_type: MarketType,
        filter_fn: F,
    ) -> Result<Vec<Market>, Error>
    where
        F: Fn(&SymbolDto) -> bool,
    {
        let response = reqwest::get(&format!("{}{}", url, endpoint))
            .await?
            .json::<ExchangeInfoDto>()
            .await?;

        let markets = response
            .symbols
            .into_iter()
            .filter(filter_fn)
            .map(|symbol| {
                let mut unified_symbol = format!("{}/{}", symbol.base_asset, symbol.quote_asset);
                if market_type == MarketType::Swap {
                    unified_symbol += ":PERP"
                }
                Market {
                    unified_symbol,
                    symbol: symbol.symbol,
                    base_asset: symbol.base_asset,
                    quote_asset: symbol.quote_asset,
                    market_type,
                }
            })
            .collect();

        Ok(markets)
    }
}

// Make BinanceAdapter satisfy the ExchangeAdapter contract
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

    fn get_spot_markets(&self) -> Vec<Market> {
        self.markets.spot.clone()
    }

    fn get_swap_markets(&self) -> Vec<Market> {
        self.markets.swap.clone()
    }

    async fn load_spot_markets(&self) -> Result<Vec<Market>, anyhow::Error> {
        self.fetch_and_map_markets(
            &self.api.spot_url,
            "/api/v3/exchangeInfo",
            MarketType::Spot,
            |symbol| symbol.quote_asset == "USDT",
        )
        .await
    }

    async fn load_swap_markets(&self) -> Result<Vec<Market>, Error> {
        self.fetch_and_map_markets(
            &self.api.swap_url,
            "/fapi/v1/exchangeInfo",
            MarketType::Swap,
            |symbol| {
                symbol.quote_asset == "USDT"
                    && symbol.contract_type == Some("PERPETUAL".to_string())
            },
        )
        .await
    }

    #[instrument(level = "info", skip(self))]
    async fn load_markets(&mut self) -> Result<Vec<Market>, Error> {
        let (spot_markets, swap_markets) =
            tokio::join!(self.load_spot_markets(), self.load_swap_markets());

        // Store the results in the struct
        self.markets.spot = spot_markets?;
        self.markets.swap = swap_markets?;

        // Return combined markets
        let mut all_markets = self.markets.spot.clone();
        all_markets.extend(self.markets.swap.clone());

        Ok(all_markets)
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

        let response = request.send().await?.json::<Vec<BookTickerDto>>().await?;
        let tickers = response
            .iter()
            .filter(|book_ticker| book_ticker.symbol.ends_with("USDT"))
            .map(|book_ticker| Ticker24HrChange {
                unified_symbol: self.unwrap_symbol(book_ticker.symbol.clone(), MarketType::Spot),
                symbol: book_ticker.symbol.clone(),
                percentage_change: book_ticker.percentage_change.parse().unwrap_or(0.0),
            })
            .collect();

        Ok(tickers)
    }

    // TODO: fix when return not vector but singular ticker

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
        let tickers = response
            .iter()
            .filter(|book_ticker| book_ticker.symbol.ends_with("USDT"))
            .map(|book_ticker| Ticker24HrChange {
                unified_symbol: self.unwrap_symbol(book_ticker.symbol.clone(), MarketType::Swap),
                symbol: book_ticker.symbol.clone(),
                percentage_change: book_ticker.percentage_change.parse().unwrap_or(0.0),
            })
            .collect();
        Ok(tickers)
    }

    async fn fetch_tickers(
        &self,
        tickers: Option<Vec<String>>,
    ) -> Result<Vec<Ticker24HrChange>, Error> {
        let (spot_tickers, swap_tickers) =
            tokio::join!(self.fetch_spot_tickers(None), self.fetch_swap_tickers(None));

        // Return combined markets
        let mut all_tickers: Vec<Ticker24HrChange> = spot_tickers?.clone();
        all_tickers.extend(swap_tickers?);
        Ok(all_tickers)
    }
}

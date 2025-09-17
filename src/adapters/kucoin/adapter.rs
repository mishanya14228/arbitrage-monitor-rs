use crate::adapters::common::types::{
    ExchangeApiConfig, ExchangeMarketsInfo, MarketType, TickerBidAsk,
};
use crate::adapters::kucoin::types::{KucoinSpotTickersListDto, KucoinSwapTickersListDto};
use crate::adapters::kucoin::KucoinContractsListDto;
use crate::adapters::ExchangeAdapter;
use crate::common::parse_json_response;
use anyhow::Error;
use async_trait::async_trait;
use polars::datatypes::PlSmallStr;
use polars::prelude::{col, JoinArgs, JoinType, LazyFrame};
use serde::de::DeserializeOwned;

pub struct KucoinAdapter {
    markets: ExchangeMarketsInfo,
    api: ExchangeApiConfig,
}

impl KucoinAdapter {
    pub fn new() -> KucoinAdapter {
        Self {
            api: {
                ExchangeApiConfig {
                    spot_url: "https://api.kucoin.com".to_string(),
                    swap_url: "https://api-futures.kucoin.com".to_string(),
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

    async fn http_get<T>(&self, url: String) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let raw_response = reqwest::get(url).await?;
        parse_json_response::<T>(raw_response).await
    }

    async fn fetch_swap_tickers_market(&self) -> Result<KucoinContractsListDto, Error> {
        const ENDPOINT: &str = "/api/v1/contracts/active";
        let url = format!("{}{}", self.api.swap_url, ENDPOINT);
        self.http_get(url).await
    }
}

#[async_trait]
impl ExchangeAdapter for KucoinAdapter {
    fn markets(&self) -> &ExchangeMarketsInfo {
        &self.markets
    }

    fn markets_mut(&mut self) -> &mut ExchangeMarketsInfo {
        &mut self.markets
    }

    fn name(&self) -> &str {
        "Kucoin"
    }

    fn wrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        match market_type {
            MarketType::Swap => match symbol.as_str() {
                "BTC/USDT:PERP" => String::from("XBTUSDTM"),
                "NEIRO/USDT:PERP" => String::from("NEIROCTOUSDTM"),
                _ => format!("{}USDTM", symbol.strip_suffix("/USDT:PERP").unwrap_or("")),
            },
            MarketType::Spot => {
                format!("{}-USDT", symbol.strip_suffix("/USDT").unwrap_or(""))
            }
        }
    }

    fn unwrap_symbol(&self, symbol: String, market_type: MarketType) -> String {
        match market_type {
            MarketType::Swap => match symbol.as_str() {
                "XBTUSDTM" => String::from("BTC/USDT:PERP"),
                "NEIROCTOUSDTM" => String::from("NEIRO/USDT:PERP"),
                _ => format!("{}/USDT:PERP", symbol.strip_suffix("USDTM").unwrap_or("")),
            },
            MarketType::Spot => {
                let base = symbol.strip_suffix("-USDT").unwrap_or("");
                format!("{}/USDT", base)
            }
        }
    }

    async fn fetch_spot_tickers(
        &self,
        _tickers: Option<Vec<&str>>,
    ) -> Result<Vec<TickerBidAsk>, Error> {
        const ENDPOINT: &str = "/api/v1/market/allTickers";
        let url = format!("{}{}", self.api.spot_url, ENDPOINT);
        let response: KucoinSpotTickersListDto = self.http_get(url).await?;
        let tickers = response
            .data
            .ticker
            .iter()
            .filter(|ticker| ticker.symbol.ends_with("-USDT"))
            .map(|book_ticker| {
                let mut ticker: TickerBidAsk = book_ticker.into();
                ticker.set_unified_symbol(
                    self.unwrap_symbol(book_ticker.symbol.clone(), MarketType::Spot),
                );
                ticker
            })
            .collect();
        Ok(tickers)
    }

    async fn fetch_swap_tickers(&self, _tickers: Option<&str>) -> Result<Vec<TickerBidAsk>, Error> {
        const ENDPOINT: &str = "/api/v1/allTickers";
        let url = format!("{}{}", self.api.swap_url, ENDPOINT);
        let (tickers_response, contracts_response) = tokio::join!(
            self.http_get::<KucoinSwapTickersListDto>(url),
            self.fetch_swap_tickers_market()
        );
        let tickers_df: LazyFrame = tickers_response?.into();
        let contracts_df: LazyFrame = contracts_response?.into();

        let joined_df = tickers_df
            .join(
                contracts_df,
                [col("symbol")],
                [col("symbol")],
                JoinArgs::new(JoinType::Inner).with_suffix(Some(PlSmallStr::from_str("_long"))),
            )
            .collect()?;

        let mut tickers: Vec<TickerBidAsk> = vec![];
        let symbol_col = joined_df.column("symbol")?;
        let bid_col = joined_df.column("bid")?;
        let ask_col = joined_df.column("ask")?;
        let volume_usd_col = joined_df.column("volume_usd")?;

        for i in 0..joined_df.height() {
            let symbol = symbol_col.str()?.get(i).unwrap().to_string();
            let is_usdt = symbol.ends_with("USDTM");
            let is_blacklisted = symbol.eq("NEIROUSDTM");
            if !is_usdt || is_blacklisted {
                continue;
            }
            let unified_symbol = self.unwrap_symbol(symbol.clone(), MarketType::Swap);
            tickers.push(TickerBidAsk {
                symbol,
                unified_symbol,
                bid: bid_col.f32()?.get(i).unwrap(),
                ask: ask_col.f32()?.get(i).unwrap(),
                volume_usd: volume_usd_col.f32()?.get(i).unwrap(),
            });
        }
        Ok(tickers)
    }
}

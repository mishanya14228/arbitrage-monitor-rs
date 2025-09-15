use super::types::{BitgetApiResponse, BitgetTickerDto};
use crate::adapters::common::{
    traits::ExchangeAdapter,
    types::{ExchangeApiConfig, ExchangeMarketsInfo, MarketType, TickerBidAsk},
};
use crate::common::parse_json_response;
use anyhow::Error;
use async_trait::async_trait;

pub struct BitgetAdapter {
    markets: ExchangeMarketsInfo,
    api: ExchangeApiConfig,
}

impl BitgetAdapter {
    pub fn new() -> Self {
        const BASE_URL: &str = "https://api.bitget.com";
        BitgetAdapter {
            api: ExchangeApiConfig {
                spot_url: BASE_URL.to_string(),
                swap_url: BASE_URL.to_string(),
            },
            markets: ExchangeMarketsInfo {
                spot: Vec::new(),
                swap: Vec::new(),
            },
        }
    }

    pub fn map_tickers(
        &self,
        response: Vec<BitgetTickerDto>,
        market_type: MarketType,
    ) -> Vec<TickerBidAsk> {
        response
            .iter()
            .filter(|ticker| {
                ticker.symbol.ends_with("USDT") && ticker.bid.is_some() && ticker.ask.is_some()
            })
            .map(|ticker| {
                let mut ticker_bid_ask: TickerBidAsk = ticker.into();
                ticker_bid_ask
                    .set_unified_symbol(self.unwrap_symbol(ticker.symbol.clone(), market_type));
                ticker_bid_ask
            })
            .collect()
    }
}

#[async_trait]
impl ExchangeAdapter for BitgetAdapter {
    fn markets(&self) -> &ExchangeMarketsInfo {
        &self.markets
    }

    fn markets_mut(&mut self) -> &mut ExchangeMarketsInfo {
        &mut self.markets
    }

    fn name(&self) -> &str {
        "Bitget"
    }

    async fn fetch_spot_tickers(
        &self,
        _tickers: Option<Vec<&str>>,
    ) -> Result<Vec<TickerBidAsk>, Error> {
        const ENDPOINT: &str = "/api/v2/spot/market/tickers";
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api.spot_url, ENDPOINT);

        let raw_response = client.get(&url).send().await?;
        let api_response: BitgetApiResponse<BitgetTickerDto> =
            parse_json_response(raw_response).await?;
        let tickers = self.map_tickers(api_response.data, MarketType::Spot);
        Ok(tickers)
    }

    async fn fetch_swap_tickers(&self, _ticker: Option<&str>) -> Result<Vec<TickerBidAsk>, Error> {
        const ENDPOINT: &str = "/api/v2/mix/market/tickers";
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api.swap_url, ENDPOINT);

        let raw_response = client
            .get(&url)
            .query(&[("productType", "USDT-FUTURES")])
            .send()
            .await?;
        let api_response: BitgetApiResponse<BitgetTickerDto> =
            parse_json_response(raw_response).await?;
        let tickers = self.map_tickers(api_response.data, MarketType::Swap);
        Ok(tickers)
    }
}

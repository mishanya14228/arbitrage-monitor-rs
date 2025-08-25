use super::types::{ExchangeInfoDto, SymbolDto};
use crate::adapters::common::{
    traits::ExchangeAdapter,
    types::{ExchangeMarketsInfo, Market, MarketType},
};
use anyhow::Error;
use tokio;
use tracing::instrument;

// Just data - no methods yet
#[derive(Debug)]
pub struct BinanceAdapter {
    spot_url: String,
    swap_url: String,
    pub markets: ExchangeMarketsInfo,
}

// Add methods to BinanceAdapter
impl BinanceAdapter {
    pub fn new() -> Self {
        BinanceAdapter {
            spot_url: "https://api.binance.com".to_string(),
            swap_url: "https://fapi.binance.com".to_string(),
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
            .map(|symbol| Market {
                unified_symbol: format!("{}/{}", symbol.base_asset, symbol.quote_asset),
                symbol: symbol.symbol,
                base_asset: symbol.base_asset,
                quote_asset: symbol.quote_asset,
                market_type,
            })
            .collect();

        Ok(markets)
    }
}

// Make BinanceAdapter satisfy the ExchangeAdapter contract
impl ExchangeAdapter for BinanceAdapter {
    fn name(&self) -> &str {
        "Binance"
    }
    
    async fn get_spot_markets(&self) -> Result<Vec<Market>, anyhow::Error> {
        self.fetch_and_map_markets(
            &self.spot_url,
            "/api/v3/exchangeInfo",
            MarketType::Spot,
            |symbol| symbol.quote_asset == "USDT",
        )
        .await
    }

    async fn get_swap_markets(&self) -> Result<Vec<Market>, Error> {
        self.fetch_and_map_markets(
            &self.swap_url,
            "/fapi/v1/exchangeInfo",
            MarketType::Swap,
            |symbol| {
                symbol.quote_asset == "USDT"
                    && symbol.contract_type == Some("PERPETUAL".to_string())
            },
        )
        .await

        // const ENDPOINT: &str = "/fapi/v1/exchangeInfo";
        // let response = reqwest::get(&format!("{}{}", self.swap_url, ENDPOINT))
        //     .await?
        //     .json::<ExchangeInfoDto>()
        //     .await?;
        //
        // let markets = response
        //     .symbols
        //     .into_iter()
        //     .filter(|symbol| {
        //         symbol.quote_asset == "USDT"
        //             && symbol.contract_type == Some("PERPETUAL".to_string())
        //     })
        //     .map(|symbol| Market {
        //         unified_symbol: format!("{}/{}", symbol.base_asset, symbol.quote_asset),
        //         symbol: symbol.symbol,
        //         base_asset: symbol.base_asset,
        //         quote_asset: symbol.quote_asset,
        //         market_type: MarketType::Swap, // These are swap/perpetual contracts
        //     })
        //     .collect();
        // Ok(markets)
    }

    #[instrument(level = "info", skip(self))]
    async fn load_markets(&mut self) -> Result<Vec<Market>, Error> {
        let (spot_markets, swap_markets) =
            tokio::join!(self.get_spot_markets(), self.get_swap_markets());

        // Store the results in the struct
        self.markets.spot = spot_markets?;
        self.markets.swap = swap_markets?;

        // Return combined markets
        let mut all_markets = self.markets.spot.clone();
        all_markets.extend(self.markets.swap.clone());

        Ok(all_markets)
    }
}

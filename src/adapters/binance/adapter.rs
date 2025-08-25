use super::types::ExchangeInfoDto;
use crate::adapters::common::{
    traits::ExchangeAdapter,
    types::{Market, MarketType},
};
use tracing::{info, warn, error, debug, instrument};

// Just data - no methods yet
#[derive(Debug)]
pub struct BinanceAdapter {
    pub base_url: String,
}

// Add methods to BinanceAdapter
impl BinanceAdapter {
    pub fn new() -> Self {
        BinanceAdapter {
            base_url: "https://api.binance.com".to_string(),
        }
    }
}

// Make BinanceAdapter satisfy the ExchangeAdapter contract
impl ExchangeAdapter for BinanceAdapter {

    #[instrument(level = "info", skip(self))]
    async fn get_markets(&self) -> Result<Vec<Market>, anyhow::Error> {
        const ENDPOINT: &str = "/api/v3/exchangeInfo";
        let response = reqwest::get(&format!("{}{}", self.base_url, ENDPOINT))
            .await?
            .json::<ExchangeInfoDto>()
            .await?;

        let markets = response.symbols
            .into_iter()
            .filter(|symbol| symbol.quote_asset == "USDT")
            .map(|symbol| Market {
                unified_symbol: format!("{}/{}", symbol.base_asset, symbol.quote_asset),
                symbol: symbol.symbol,
                base_asset: symbol.base_asset,
                quote_asset: symbol.quote_asset,
                market_type: MarketType::Spot,  // Binance /exchangeInfo only returns spot markets
            })
            .collect();

        Ok(markets)


        // .json::<ExchangeInfoDto>()
        // .await?
        // vec![
        //     Market {
        //         symbol: "BTCUSDT".to_string(),
        //         base_asset: "BTC".to_string(),
        //         quote_asset: "USDT".to_string(),
        //         market_type: MarketType::Spot,
        //     }
        // ]
    }
    fn name(&self) -> &str {
        "Binance"
    }
}

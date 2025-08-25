use super::types::{Market, MarketType, SimpleTicker};

// This is like an interface - defines what all exchanges must implement
pub trait ExchangeAdapter {
    // Every exchange must have this method
    async fn get_spot_markets(&self) -> Result<Vec<Market>, anyhow::Error>;
    async fn get_swap_markets(&self) -> Result<Vec<Market>, anyhow::Error>;
    async fn load_markets(&mut self) -> Result<Vec<Market>, anyhow::Error>;

    fn wrap_symbol(&self, symbol: String, market_type: MarketType) -> String;
    fn unwrap_symbol(&self, symbol: String, market_type: MarketType) -> String;

    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<String>>,
    ) -> Result<Vec<SimpleTicker>, anyhow::Error>;
    async fn fetch_swap_tickers(
        &self,
        tickers: Option<Vec<String>>,
    ) -> Result<Vec<SimpleTicker>, anyhow::Error>;
    async fn fetch_tickers(
        &self,
        tickers: Option<Vec<String>>,
    ) -> Result<Vec<SimpleTicker>, anyhow::Error>;

    fn name(&self) -> &str;
}

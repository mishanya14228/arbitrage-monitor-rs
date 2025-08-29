use super::types::{Market, MarketType, Ticker24HrChange};
use async_trait::async_trait;

// This is like an interface - defines what all exchanges must implement
#[async_trait]
pub trait ExchangeAdapter {
    fn name(&self) -> &str;

    fn wrap_symbol(&self, symbol: String, market_type: MarketType) -> String;
    fn unwrap_symbol(&self, symbol: String, market_type: MarketType) -> String;

    fn get_spot_markets(&self) -> Vec<Market>;
    fn get_swap_markets(&self) -> Vec<Market>;

    async fn load_spot_markets(&self) -> Result<Vec<Market>, anyhow::Error>;
    async fn load_swap_markets(&self) -> Result<Vec<Market>, anyhow::Error>;
    async fn load_markets(&mut self) -> Result<Vec<Market>, anyhow::Error>;

    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<&str>>,
    ) -> Result<Vec<Ticker24HrChange>, anyhow::Error>;
    async fn fetch_swap_tickers(
        &self,
        tickers: Option<&str>,
    ) -> Result<Vec<Ticker24HrChange>, anyhow::Error>;
    async fn fetch_tickers(
        &self,
        tickers: Option<Vec<String>>,
    ) -> Result<Vec<Ticker24HrChange>, anyhow::Error>;
}

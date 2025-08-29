use super::types::{MarketType, Ticker24HrChange};
use async_trait::async_trait;

#[async_trait]
pub trait ExchangeAdapter {
    fn name(&self) -> &str;

    fn wrap_symbol(&self, symbol: String, market_type: MarketType) -> String;
    fn unwrap_symbol(&self, symbol: String, market_type: MarketType) -> String;

    fn get_spot_markets(&self) -> Vec<Ticker24HrChange>;
    fn get_swap_markets(&self) -> Vec<Ticker24HrChange>;

    async fn fetch_spot_tickers(
        &self,
        tickers: Option<Vec<&str>>,
    ) -> Result<Vec<Ticker24HrChange>, anyhow::Error>;
    async fn fetch_swap_tickers(
        &self,
        tickers: Option<&str>,
    ) -> Result<Vec<Ticker24HrChange>, anyhow::Error>;
    async fn fetch_tickers(&self) -> Result<Vec<Ticker24HrChange>, anyhow::Error>;
}

use super::types::Market;

// This is like an interface - defines what all exchanges must implement
pub trait ExchangeAdapter {
    // Every exchange must have this method
    async fn get_spot_markets(&self) -> Result<Vec<Market>, anyhow::Error>;
    async fn get_swap_markets(&self) -> Result<Vec<Market>, anyhow::Error>;
    async fn load_markets(&mut self) -> Result<Vec<Market>, anyhow::Error>;

    // Every exchange must have a name
    fn name(&self) -> &str;
}

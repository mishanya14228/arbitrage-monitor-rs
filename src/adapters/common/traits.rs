use super::types::Market;

// This is like an interface - defines what all exchanges must implement
pub trait ExchangeAdapter {
    // Every exchange must have this method
    async fn get_markets(&self) -> Result<Vec<Market>, anyhow::Error>;

    // Every exchange must have a name
    fn name(&self) -> &str;
}

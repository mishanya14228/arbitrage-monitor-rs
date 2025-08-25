use super::types::Market;

// This is like an interface - defines what all exchanges must implement
pub trait ExchangeAdapter {
    // Every exchange must have this method
    fn get_markets(&self) -> Vec<Market>;
    
    // Every exchange must have a name
    fn name(&self) -> &str;
}
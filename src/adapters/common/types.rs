#[derive(Debug, Clone)]
pub struct ExchangeMarketsInfo {
    pub spot: Vec<Market>,
    pub swap: Vec<Market>,
}

impl ExchangeMarketsInfo {
    pub fn get_markets(&self, market_type: MarketType) -> &Vec<Market> {
        match market_type {
            MarketType::Spot => &self.spot,
            MarketType::Swap => &self.swap,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Market {
    pub symbol: String,      // "BTCUSDT"
    pub base_asset: String,  // "BTC"
    pub quote_asset: String, // "USDT"
    pub unified_symbol: String,
    pub market_type: MarketType,
}

#[derive(Debug, Clone, Copy)]
pub enum MarketType {
    Spot,
    Swap,
}

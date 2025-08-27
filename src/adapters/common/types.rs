#[derive(Debug, Clone)]
pub struct ExchangeApiConfig {
    pub spot_url: String,
    pub swap_url: String,
}

#[derive(Debug, Clone)]
pub struct ExchangeMarketsInfo {
    pub spot: Vec<Market>,
    pub swap: Vec<Market>,
}

impl ExchangeMarketsInfo {
    #[allow(dead_code)]
    pub fn get_markets(&self, market_type: MarketType) -> &Vec<Market> {
        match market_type {
            MarketType::Spot => &self.spot,
            MarketType::Swap => &self.swap,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Market {
    pub symbol: String,      // "BTCUSDT"
    pub base_asset: String,  // "BTC"
    pub quote_asset: String, // "USDT"
    pub unified_symbol: String,
    pub market_type: MarketType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarketType {
    Spot,
    Swap,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Ticker24HrChange {
    pub symbol: String,
    pub unified_symbol: String,
    pub percentage_change: f32,
}

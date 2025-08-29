#[derive(Debug, Clone)]
pub struct ExchangeApiConfig {
    pub spot_url: String,
    pub swap_url: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Ticker24HrChange {
    pub symbol: String,
    pub unified_symbol: String,
    pub percentage_change: f32,
}

#[derive(Debug, Clone)]
pub struct ExchangeMarketsInfo {
    pub spot: Vec<Ticker24HrChange>,
    pub swap: Vec<Ticker24HrChange>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarketType {
    Spot,
    Swap,
}

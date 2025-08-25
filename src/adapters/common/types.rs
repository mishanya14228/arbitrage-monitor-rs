// This is like a TypeScript interface or Python dataclass
#[derive(Debug, Clone)]
pub struct Market {
    pub symbol: String,      // "BTCUSDT"
    pub base_asset: String,  // "BTC"
    pub quote_asset: String, // "USDT"
    pub unified_symbol: String,
    pub market_type: MarketType,
}

#[derive(Debug, Clone)]
pub enum MarketType {
    Spot,
    Futures,
}
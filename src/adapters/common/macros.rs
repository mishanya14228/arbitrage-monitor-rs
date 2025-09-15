#[macro_export]
macro_rules! impl_ticker_from {
    ($dto_type:ty) => {
        impl From<&$dto_type> for TickerBidAsk {
            fn from(dto: &$dto_type) -> Self {
                TickerBidAsk {
                    unified_symbol: dto.symbol.clone(),
                    symbol: dto.symbol.clone(),
                    volume_usd: dto.volume.parse().unwrap_or(0.0),
                    bid: dto.bid.parse().unwrap_or(0.0),
                    ask: dto.ask.parse().unwrap_or(0.0),
                }
            }
        }
    };
}

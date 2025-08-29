use crate::adapters::{BinanceAdapter, BybitAdapter, ExchangeAdapter};
#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ExchangeType {
    Binance,
    Bybit,
    // Bitget,
    // OKX,
    // BingX,
    // Gate,
    // Kucoin,
    // HTX,
    // MEXC,
    // WhiteBit,
}

impl ExchangeType {
    pub fn create_adapter(&self) -> Box<dyn ExchangeAdapter> {
        match self {
            ExchangeType::Binance => Box::new(BinanceAdapter::new()),
            ExchangeType::Bybit => Box::new(BybitAdapter::new()), // _ => Box::new(BinanceAdapter::new()),
                                                                  // ExchangeType::Bitget => {}
                                                                  // ExchangeType::OKX => {}
                                                                  // ExchangeType::BingX => {}
                                                                  // ExchangeType::Gate => {}
                                                                  // ExchangeType::Kucoin => {}
                                                                  // ExchangeType::HTX => {}
                                                                  // ExchangeType::MEXC => {}
                                                                  // ExchangeType::WhiteBit => {}
        }
    }
}

use crate::adapters::{BinanceAdapter, BitgetAdapter, BybitAdapter, ExchangeAdapter, GateAdapter, KucoinAdapter, OkxAdapter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ExchangeType {
    Binance,
    Bybit,
    OKX,
    Gate,
    Bitget,
    Kucoin,
    // BingX,
    // HTX,
    // MEXC,
    // WhiteBit,
}

impl ExchangeType {
    pub fn create_adapter(&self) -> Box<dyn ExchangeAdapter> {
        match self {
            ExchangeType::Binance => Box::new(BinanceAdapter::new()),
            ExchangeType::Bybit => Box::new(BybitAdapter::new()),
            ExchangeType::OKX => Box::new(OkxAdapter::new()),
            ExchangeType::Gate => Box::new(GateAdapter::new()),
            ExchangeType::Bitget => Box::new(BitgetAdapter::new()),
            ExchangeType::Kucoin => Box::new(KucoinAdapter::new()),
            // ExchangeType::BingX => {}
            // ExchangeType::HTX => {}
            // ExchangeType::MEXC => {}
            // ExchangeType::WhiteBit => {}
            // _ => Box::new(BinanceAdapter::new()),
        }
    }
}

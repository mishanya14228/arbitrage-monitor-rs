use crate::adapters::okx::OkxTickerDto;

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

impl From<&OkxTickerDto> for Ticker24HrChange {
    fn from(dto: &OkxTickerDto) -> Self {
        let last_price = dto.last.parse().unwrap_or(0.0);
        let open_price = dto.open24h.parse().unwrap_or(0.0);

        let percentage_change = if open_price != 0.0 {
            ((last_price - open_price) / open_price) * 100.0
        } else {
            0.0
        };

        Ticker24HrChange {
            symbol: dto.symbol.clone(),
            unified_symbol: dto.symbol.clone(),
            percentage_change,
        }
    }
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

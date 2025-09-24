use crate::adapters::binance::BinanceBookTickerDto;
use crate::adapters::bitget::BitgetTickerDto;
use crate::adapters::bybit::BybitTickerDto;
use crate::adapters::gate::{GateSpotTickerDto, GateSwapTickerDto};
use crate::adapters::kucoin::KucoinSpotTickerDto;
use crate::adapters::okx::OkxTickerDto;
use crate::adapters::ExchangeAdapter;
use crate::common::ExchangeType;
use crate::impl_ticker_from;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub type AdaptersMap = HashMap<ExchangeType, Box<dyn ExchangeAdapter>>;

#[derive(Debug, Clone)]
pub struct ExchangeApiConfig {
    pub spot_url: String,
    pub swap_url: String,
    // pub ws_spot_url: String,
    pub ws_swap_url: String,
}

#[derive(Debug, Clone)]
pub struct ExchangeMarketsInfo {
    pub spot: Vec<TickerBidAsk>,
    pub swap: Vec<TickerBidAsk>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarketType {
    Spot,
    Swap,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TickerBidAsk {
    pub symbol: String,
    pub unified_symbol: String,
    pub bid: f32,
    pub ask: f32,
    pub volume_usd: f32,
}

impl TickerBidAsk {
    pub fn get_market_type(&self) -> MarketType {
        if self.unified_symbol.contains("PERP") {
            MarketType::Swap
        } else {
            MarketType::Spot
        }
    }

    pub fn set_unified_symbol(&mut self, symbol: String) {
        self.unified_symbol = symbol;
    }
}

impl Display for TickerBidAsk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.unified_symbol, self.volume_usd)
    }
}

impl From<&BybitTickerDto> for TickerBidAsk {
    fn from(dto: &BybitTickerDto) -> Self {
        let bid = dto.bid.parse().unwrap_or(0.0);
        let coin_volume = dto.volume_coin.parse().unwrap_or(0.0);
        TickerBidAsk {
            unified_symbol: dto.symbol.clone(),
            symbol: dto.symbol.clone(),
            volume_usd: coin_volume * bid,
            bid,
            ask: dto.ask.parse().unwrap_or(0.0),
        }
    }
}

impl From<&BitgetTickerDto> for TickerBidAsk {
    fn from(dto: &BitgetTickerDto) -> Self {
        TickerBidAsk {
            unified_symbol: dto.symbol.clone(),
            symbol: dto.symbol.clone(),
            volume_usd: dto.volume.parse().unwrap_or(0.0),
            bid: dto.bid.as_ref().and_then(|b| b.parse().ok()).unwrap_or(0.0),
            ask: dto.ask.as_ref().and_then(|a| a.parse().ok()).unwrap_or(0.0),
        }
    }
}

impl_ticker_from!(OkxTickerDto);
impl_ticker_from!(BinanceBookTickerDto);
impl_ticker_from!(GateSpotTickerDto);
impl_ticker_from!(GateSwapTickerDto);
impl_ticker_from!(KucoinSpotTickerDto);

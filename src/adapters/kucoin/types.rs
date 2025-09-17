use polars::df;
use polars::prelude::{IntoLazy, LazyFrame};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KucoinSwapTickersListDto {
    pub data: Vec<KucoinSwapTickerDto>,
}

impl Into<LazyFrame> for KucoinSwapTickersListDto {
    fn into(self) -> LazyFrame {
        let mut symbols = Vec::new();
        let mut bids = Vec::new();
        let mut asks = Vec::new();
        self.data.into_iter().for_each(|d| {
            symbols.push(d.symbol);
            bids.push(d.bid.parse::<f32>().unwrap());
            asks.push(d.ask.parse::<f32>().unwrap());
        });
        df![
            "symbol" => symbols,
            "bid" => bids,
            "ask" => asks,
        ]
        .unwrap()
        .lazy()
    }
}

#[derive(Serialize, Deserialize)]
pub struct KucoinSwapTickerDto {
    pub symbol: String,

    #[serde(rename = "bestBidPrice")]
    pub bid: String,

    #[serde(rename = "bestAskPrice")]
    pub ask: String,
}

#[derive(Serialize, Deserialize)]
pub struct KucoinSpotTickersListDto {
    pub data: KucoinSpotTickersWrapDto,
}

#[derive(Serialize, Deserialize)]
pub struct KucoinSpotTickersWrapDto {
    pub ticker: Vec<KucoinSpotTickerDto>,
}

#[derive(Serialize, Deserialize)]
pub struct KucoinSpotTickerDto {
    pub symbol: String,

    #[serde(rename = "buy")]
    pub bid: String,

    #[serde(rename = "sell")]
    pub ask: String,

    #[serde(rename = "volValue")]
    pub volume: String,
}

#[derive(Serialize, Deserialize)]
pub struct KucoinContractsListDto {
    data: Vec<KucoinContractDto>,
}

#[derive(Serialize, Deserialize)]
pub struct KucoinContractDto {
    symbol: String,

    #[serde(rename = "volumeOf24h")]
    volume_coin: f32,

    #[serde(rename = "lastTradePrice")]
    last_price: f32,
}

impl Into<LazyFrame> for KucoinContractsListDto {
    fn into(self) -> LazyFrame {
        let mut symbols = Vec::new();
        let mut volume_usd = Vec::new();
        self.data.into_iter().for_each(|d| {
            symbols.push(d.symbol);
            let volume = d.volume_coin * d.last_price;
            volume_usd.push(volume);
        });
        df![
            "symbol" => symbols,
            "volume_usd" => volume_usd,
        ]
        .unwrap()
        .lazy()
    }
}

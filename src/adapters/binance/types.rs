use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BinanceExchangeInfoDto {
    pub symbols: Vec<BinanceSymbolDto>,
}

#[derive(Serialize, Deserialize)]
pub struct BinanceSymbolDto {
    pub symbol: String,

    #[serde(rename = "baseAsset")]
    pub base_asset: String,

    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BinanceBookTickerDto {
    pub symbol: String,

    #[serde(rename = "priceChangePercent")]
    pub percentage_change: String,

    #[serde(rename = "lastPrice")]
    pub last_price: String,

    #[serde(rename = "quoteVolume")]
    pub volume: String,
}

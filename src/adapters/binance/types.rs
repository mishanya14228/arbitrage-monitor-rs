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

fn default_volume() -> String {
    "10000000".to_string()
}

#[derive(Serialize, Deserialize)]
pub struct BinanceBookTickerDto {
    pub symbol: String,

    #[serde(rename = "quoteVolume", default = "default_volume")]
    pub volume: String,

    #[serde(rename = "bidPrice")]
    pub bid: String,

    #[serde(rename = "askPrice")]
    pub ask: String,
}

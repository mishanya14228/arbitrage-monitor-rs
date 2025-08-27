use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExchangeInfoDto {
    pub symbols: Vec<SymbolDto>,
}

#[derive(Serialize, Deserialize)]
pub struct SymbolDto {
    pub symbol: String,

    #[serde(rename = "baseAsset")]
    pub base_asset: String,

    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    #[serde(rename = "contractType")]
    pub contract_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BookTickerDto {
    pub symbol: String,

    #[serde(rename = "priceChangePercent")]
    pub percentage_change: String,
}

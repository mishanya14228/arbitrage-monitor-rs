use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExchangeInfoDto {
    // timezone: String,
    // server_time: i64,
    // rate_limits: serde_json::Value,
    // exchange_filters: Vec<Option<serde_json::Value>>,
    pub symbols: Vec<SymbolDto>
}

#[derive(Serialize, Deserialize)]
pub struct SymbolDto {
    pub symbol: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
}

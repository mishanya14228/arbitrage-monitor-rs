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
    
    #[serde(rename = "bidPrice")]
    pub bid_price: String,
    
    #[serde(rename = "bidQty")]
    pub bid_qty: String,
    
    #[serde(rename = "askPrice")]
    pub ask_price: String,

    #[serde(rename = "askQty")]
    pub ask_qty: String
}

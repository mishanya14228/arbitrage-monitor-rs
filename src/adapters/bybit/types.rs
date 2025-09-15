use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BybitTickersListDto {
    pub result: Result,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub category: String,
    pub list: Vec<BybitTickerDto>,
}

#[derive(Serialize, Deserialize)]
pub struct BybitTickerDto {
    pub symbol: String,

    #[serde(rename = "price24hPcnt")]
    pub percentage_change: String,

    #[serde(rename = "lastPrice")]
    pub last_price: String,

    #[serde(rename = "volume24h")]
    pub volume_coin: String,
}

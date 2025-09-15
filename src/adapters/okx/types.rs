use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OkxTickersListDto {
    pub data: Vec<OkxTickerDto>,
}

#[derive(Serialize, Deserialize)]
pub struct OkxTickerDto {
    #[serde(rename = "instId")]
    pub symbol: String,

    #[serde(rename = "volCcy24h")]
    pub volume: String,

    #[serde(rename = "bidPx")]
    pub bid: String,

    #[serde(rename = "askPx")]
    pub ask: String,
}

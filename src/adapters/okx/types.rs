use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OkxTickersListDto {
    pub data: Vec<OkxTickerDto>,
}

#[derive(Serialize, Deserialize)]
pub struct OkxTickerDto {
    #[serde(rename = "instId")]
    pub symbol: String,
    pub last: String,
    pub open24h: String,

    #[serde(rename = "volCcy24h")]
    pub volume: String,
}

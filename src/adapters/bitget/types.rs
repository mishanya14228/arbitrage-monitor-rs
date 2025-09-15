use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BitgetApiResponse<T> {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct BitgetTickerDto {
    pub symbol: String,

    #[serde(rename = "bidPr")]
    pub bid: Option<String>,

    #[serde(rename = "askPr")]
    pub ask: Option<String>,

    #[serde(rename = "quoteVolume")]
    pub volume: String,
}

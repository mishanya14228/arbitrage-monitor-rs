use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GateSpotTickerDto {
    #[serde(rename = "currency_pair")]
    pub symbol: String,

    #[serde(rename = "highest_bid")]
    pub bid: String,

    #[serde(rename = "lowest_ask")]
    pub ask: String,

    #[serde(rename = "quote_volume")]
    pub volume: String,
}

#[derive(Serialize, Deserialize)]
pub struct GateSwapTickerDto {
    #[serde(rename = "contract")]
    pub symbol: String,

    #[serde(rename = "highest_bid")]
    pub bid: String,

    #[serde(rename = "lowest_ask")]
    pub ask: String,

    #[serde(rename = "volume_24h_quote")]
    pub volume: String,
}

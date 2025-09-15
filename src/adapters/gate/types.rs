use serde::{Deserialize, Serialize};

pub type GateSpotTickersListDto = Vec<GateSpotTickerDto>;

#[derive(Serialize, Deserialize)]
pub struct GateSpotTickerDto {
    #[serde(rename = "currency_pair")]
    pub symbol: String,

    #[serde(rename = "change_percentage")]
    pub percentage_change: String,

    #[serde(rename = "last")]
    pub last_price: String,

    #[serde(rename = "quote_volume")]
    pub volume: String,
}

pub type GateSwapTickersListDto = Vec<GateSwapTickerDto>;

#[derive(Serialize, Deserialize)]
pub struct GateSwapTickerDto {
    #[serde(rename = "contract")]
    pub symbol: String,

    #[serde(rename = "change_percentage")]
    pub percentage_change: String,

    #[serde(rename = "last")]
    pub last_price: String,

    #[serde(rename = "volume_24h_quote")]
    pub volume: String,
}

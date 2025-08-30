use serde::{Deserialize, Serialize};

pub type GateSpotTickersListDto = Vec<GateSpotTickerDto>;

#[derive(Serialize, Deserialize)]
pub struct GateSpotTickerDto {
    #[serde(rename = "currency_pair")]
    pub symbol: String,

    #[serde(rename = "change_percentage")]
    pub percentage_change: String,
}

pub type GateSwapTickersListDto = Vec<GateSwapTickerDto>;

#[derive(Serialize, Deserialize)]
pub struct GateSwapTickerDto {
    #[serde(rename = "contract")]
    pub symbol: String,

    #[serde(rename = "change_percentage")]
    pub percentage_change: String,
}

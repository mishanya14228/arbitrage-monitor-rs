use reqwest::Response;
use serde::de::DeserializeOwned;
use tracing::{debug, info};

pub async fn parse_json_response<T: DeserializeOwned>(
    raw_response: Response,
) -> Result<T, anyhow::Error> {
    let text = raw_response.text().await?;
    info!("Response body: {}", text);
    let response: T = serde_json::from_str(&text)?;
    Ok(response)
}

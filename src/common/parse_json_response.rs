use reqwest::Response;
use serde::de::DeserializeOwned;
use tracing::warn;

pub async fn parse_json_response<T: DeserializeOwned>(
    raw_response: Response,
) -> Result<T, anyhow::Error> {
    let is_success = raw_response.status().is_success();
    let url = raw_response.url().clone();
    let text = raw_response.text().await?;
    if !is_success {
        warn!(
            "Response wasn't successful\n - Url: {}.\n - Response body: {}",
            url, text
        );
    }
    let response: T = serde_json::from_str(&text)?;
    Ok(response)
}

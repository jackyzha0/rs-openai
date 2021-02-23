
use anyhow::Result;
use reqwest::Client;
use reqwest::Response;
use serde::{Serialize};
use serde_json::json;

pub async fn rq<T: Serialize>(
    request_url: String,
    api_key: &String,
    options: T,
) -> Result<Response> {
    let options = json!(options);
    let res = Client::new()
        .post(&request_url)
        .bearer_auth(api_key)
        .json(&options)
        .send()
        .await?;
    Ok(res)
}

use crate::gpt::domain::completion;
use reqwest::{Result, Response};
use serde_json::json;
use reqwest::Client;
use serde::Serialize;

pub async fn rq<T: Serialize>(request_url: String, api_key: String, options: T) -> Result<Response> {
    let options = json!(options);
    Client::new()
        .post(&request_url)
        .bearer_auth(api_key)
        .json(&options)
        .send()
        .await
}
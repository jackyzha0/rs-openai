use crate::gpt::domain::completion;
use reqwest::Response;
use anyhow::Result;
use serde_json::json;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct APIResponse(Response);
impl APIResponse {
    pub async fn stringify<T>(self) -> Result<String> where T: Serialize + for<'de> Deserialize<'de> {
        let json = self.0.json::<T>().await?;
        Ok(serde_json::to_string(&json)?)
    }
}

pub async fn rq<T: Serialize>(request_url: String, api_key: &String, options: T) -> Result<APIResponse> {
    let options = json!(options);
    let res = APIResponse(Client::new()
        .post(&request_url)
        .bearer_auth(api_key)
        .json(&options)
        .send()
        .await?);
    Ok(res)
}
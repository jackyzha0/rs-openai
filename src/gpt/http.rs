use crate::gpt::domain::completion;
use reqwest::Result;
use serde_json::json;
use reqwest::Client;

#[tokio::main]
async fn rq() ->  Result<()> {
    let gist_body = json!({
        "description": "the description for this gist",
        "public": true,
        "files": {
             "main.rs": {
             "content": r#"fn main() { println!("hello world!");}"#
            }
        }});

    let request_url = "https://api.github.com/gists";
    let response = Client::new()
        .post(request_url)
        .json(&gist_body)
        .send().await?;

    let gist: completion::Response = response.json().await?;
    println!("Created {:?}", gist);
    Ok(())
}
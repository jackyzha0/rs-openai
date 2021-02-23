#[macro_use]
extern crate rocket;
extern crate dotenv;

use crate::gpt::domain::completion::{CompletionResponse, Options as CompletionOptions};
use crate::gpt::domain::search::{Options as SearchOptions, SearchResponse};
use crate::gpt::domain::unions::{UnionOptions, UnionResponse};
use anyhow::Error;

use rocket::response::Responder;

use crate::gpt::types::{EngineType, TaskType};
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::str;

mod gpt;

#[derive(Deserialize, Serialize, Debug, Responder)]
pub enum DomainError {
    #[response(status = 400, content_type = "plain")]
    BadRequest(String),
    #[response(status = 500, content_type = "plain")]
    Internal(String),
}

impl From<Error> for DomainError {
    fn from(error: Error) -> Self {
        DomainError::BadRequest(error.to_string())
    }
}

impl From<reqwest::Error> for DomainError {
    fn from(error: reqwest::Error) -> Self {
        DomainError::Internal(error.to_string())
    }
}

impl From<serde_json::Error> for DomainError {
    fn from(error: serde_json::Error) -> Self {
        DomainError::Internal(error.to_string())
    }
}

#[get("/healthz")]
fn healthz() -> &'static str {
    "healthy 200"
}

#[post("/search", format = "json", data = "<data>")]
async fn search(
    data: Json<SearchOptions>,
    gpt: State<'_, gpt::GPTClient>,
) -> Result<Json<SearchResponse>, DomainError> {
    let req = data.into_inner();
    let res = gpt.search(req.documents, req.query).await?;
    Ok(Json(res))
}

#[post("/summarize", format = "json", data = "<data>")]
async fn summarize(
    data: Json<CompletionOptions>,
    gpt: State<'_, gpt::GPTClient>,
) -> Result<Json<CompletionResponse>, DomainError> {
    let req = data.into_inner();
    let res = gpt.summarize(req.prompt.unwrap_or_default()).await?;
    Ok(Json(res))
}

#[post("/rephrase", format = "json", data = "<data>")]
async fn rephrase(
    data: Json<CompletionOptions>,
    gpt: State<'_, gpt::GPTClient>,
) -> Result<Json<CompletionResponse>, DomainError> {
    let req = data.into_inner();
    let res = gpt.rephrase(req.prompt.unwrap_or_default()).await?;
    Ok(Json(res))
}

#[post("/complete", format = "json", data = "<data>")]
async fn complete(
    data: Json<CompletionOptions>,
    gpt: State<'_, gpt::GPTClient>,
) -> Result<Json<CompletionResponse>, DomainError> {
    let req = data.into_inner();
    let res = gpt
        .complete(
            req.prompt.unwrap_or_default(),
            req.max_tokens.unwrap_or_default(),
        )
        .await?;
    Ok(Json(res))
}

#[post("/request/<task>/<engine>", format = "json", data = "<data>")]
async fn request(
    task: TaskType,
    engine: EngineType,
    data: Json<UnionOptions>,
    gpt: State<'_, gpt::GPTClient>,
) -> Result<Json<UnionResponse>, DomainError> {
    let req = data.into_inner();
    let res = gpt.request(engine, task, req).await?;

    if res.status().is_success() {
        let resp = res.json::<UnionResponse>().await?;
        Ok(Json(resp))
    } else {
        Err(DomainError::BadRequest(res.text().await?))
    }
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage({
            dotenv::dotenv().ok();
            let key = dotenv::var("GPT_KEY").unwrap();
            gpt::GPTClient { api_key: key }
        })
        .mount(
            "/",
            routes![healthz, search, summarize, rephrase, complete, request,],
        )
}

#[macro_use]
extern crate rocket;
extern crate dotenv;

use crate::gpt::domain::search::{Options, SearchResponse};
use anyhow::Error;

use rocket::response::Responder;

use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

mod gpt;

#[derive(Deserialize, Serialize, Debug, Responder)]
pub enum DomainError {
    #[response(status = 400, content_type = "plain")]
    BadRequest(String),
    #[response(status = 404, content_type = "plain")]
    IdNotFound(String),
}

impl From<Error> for DomainError {
    fn from(error: Error) -> Self {
        DomainError::BadRequest(error.to_string())
    }
}

#[get("/healthz")]
fn healthz() -> &'static str {
    "healthy 200"
}

#[post("/search", format = "json", data = "<data>")]
async fn search(
    data: Json<Options>,
    gpt: State<'_, gpt::GPTClient>,
) -> Result<Json<SearchResponse>, DomainError> {
    let req = data.into_inner();
    let res = gpt.search(req.documents, req.query).await?;
    Ok(Json(res))
}

#[post("/request/<task>/<engine>", format = "json", data = "<data>")]
async fn request(
    task: String,
    engine: String,
    data: Json<Options>,
    gpt: State<'_, gpt::GPTClient>,
) -> Result<Json<SearchResponse>, DomainError> {
    let req = data.into_inner();
    let res = gpt.search(req.documents, req.query).await?;
    Ok(Json(res))
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage({
            dotenv::dotenv().ok();
            let key = dotenv::var("GPT_KEY").unwrap();
            gpt::GPTClient { api_key: key }
        })
        .mount("/", routes![search, healthz,])
}
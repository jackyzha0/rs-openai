#[macro_use]
extern crate rocket;
extern crate dotenv;

use crate::gpt::domain::search::{Options, SearchResponse};
use anyhow::Error;
use rocket::response::Debug;
use rocket::response::Responder;
use rocket::response::{content, status};
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
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

// #[tokio::main]
// async fn main() {
//
//
//     let CLIENT = gpt::GPTClient{api_key: key};
//     let text = "Recently, I've been reading 21 Lessons for the 21st Century written by none other than Yuval Noah Harari, and have been enjoying the book.".parse().unwrap();
//     let res = CLIENT.rephrase(text).await;
//     println!("summarized text: {:#?}", res);
//
//     let text = "Recently, I've been reading 21 Lessons for the 21st Century written by none other than Yuval Noah Harari, and have been enjoying the book.".parse().unwrap();
//     let docs = vec![text];
//     let res = CLIENT.search(docs, "person".parse().unwrap()).await;
//     println!("matches {:#?}", res);
// }

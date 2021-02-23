use serde::{Deserialize, Serialize};
use crate::gpt::domain::completion;
use crate::gpt::domain::search;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnionOptions {
    Completion(completion::Options),
    Search(search::Options),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnionResponse {
    Completion(completion::CompletionResponse),
    Search(search::SearchResponse),
}
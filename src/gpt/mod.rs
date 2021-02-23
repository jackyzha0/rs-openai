pub(crate) mod domain;
pub(crate) mod types;
mod http;

use crate::gpt::domain::completion::{CompletionResponse, Options as CompletionOptions};
use crate::gpt::domain::search::{Options as SearchOptions, SearchResponse};
use crate::gpt::types::{EngineType, TaskType};
use crate::gpt::domain::unions::{UnionResponse, UnionOptions};
use serde::Serialize;
use anyhow::Result;
use reqwest::Response;

pub struct GPTClient {
    pub api_key: String,
}

impl GPTClient {
    pub async fn request(
        &self,
        engine: EngineType,
        task_type: TaskType,
        options: UnionOptions,
    ) -> Result<Response> {
        let res = http::rq(engine.to_endpoint(task_type), &self.api_key, options).await?;
        Ok(res)
    }

    pub async fn summarize(&self, text: String) -> Result<CompletionResponse> {
        let transform = |text| -> String {
            format!(
                "{} \
            tl;dr:",
                text
            )
        };

        let options = CompletionOptions {
            prompt: Some(transform(text)),
            max_tokens: Some(50),
            stop: Some(vec![". ".to_string()]),
            ..Default::default()
        };

        Ok(self
            .request(EngineType::Curie, TaskType::Completion, UnionOptions::Completion(options))
            .await?
            .json::<CompletionResponse>()
            .await?
        )
    }

    pub async fn rephrase(&self, text: String) -> Result<CompletionResponse> {
        let transform = |text| -> String {
            format!(
                "{}\n
            In other words: ",
                text
            )
        };

        let options = CompletionOptions {
            prompt: Some(transform(text)),
            max_tokens: Some(50),
            stop: Some(vec![". ".to_string()]),
            ..Default::default()
        };

        Ok(self
            .request(EngineType::Curie, TaskType::Completion, UnionOptions::Completion(options))
            .await?
            .json::<CompletionResponse>()
            .await?)
    }

    pub async fn complete(&self, text: String, num_tokens: u16) -> Result<CompletionResponse> {
        let options = CompletionOptions {
            prompt: Some(text),
            max_tokens: Some(num_tokens),
            stop: Some(vec![". ".to_string()]),
            ..Default::default()
        };

        Ok(self
            .request(EngineType::Davinci, TaskType::Completion, UnionOptions::Completion(options))
            .await?
            .json::<CompletionResponse>()
            .await?)
    }

    pub async fn search(&self, documents: Vec<String>, query: String) -> Result<SearchResponse> {
        let options = SearchOptions { documents, query };
        Ok(self
            .request(EngineType::Davinci, TaskType::Search, UnionOptions::Search(options))
            .await?
            .json::<SearchResponse>()
            .await?)
    }
}

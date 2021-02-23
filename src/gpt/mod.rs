pub(crate) mod domain;
mod http;
mod types;

use crate::gpt::domain::completion::{CompletionResponse, Options as CompletionOptions};
use crate::gpt::domain::search::{Options as SearchOptions, SearchResponse};
use crate::gpt::types::{EngineType, TaskType};
use serde::Serialize;
use anyhow::Result;
use reqwest::Response;

pub struct GPTClient {
    pub api_key: String,
}

impl GPTClient {
    pub async fn request<T: Serialize>(
        &self,
        engine: EngineType,
        task_type: TaskType,
        options: T,
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
            .request(EngineType::Curie, TaskType::Completion, options)
            .await?
            .json::<CompletionResponse>()
            .await?)
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
            .request(EngineType::Curie, TaskType::Completion, options)
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
            .request(EngineType::Davinci, TaskType::Completion, options)
            .await?
            .json::<CompletionResponse>()
            .await?)
    }

    pub async fn search(&self, documents: Vec<String>, query: String) -> Result<SearchResponse> {
        let options = SearchOptions { documents, query };
        Ok(self
            .request(EngineType::Davinci, TaskType::Search, options)
            .await?
            .json::<SearchResponse>()
            .await?)
    }
}

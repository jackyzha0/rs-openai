use crate::gpt::domain::completion::{CompletionResponse, Options as CompletionOptions};
use crate::gpt::domain::search::{Options as SearchOptions, SearchResponse};
use anyhow::Result;
use reqwest::Response;
use serde::Serialize;

pub(crate) mod domain;
mod http;

/// Engine Types. Davinci is the most complex and expensive, Ada is the simplest and cheapest
pub enum EngineType {
    /// Good at: Complex intent, cause and effect, summarization for audience
    Davinci,

    /// Good at: Language translation, complex classification, text sentiment, summarization
    Curie,

    /// Good at: Moderate classification, semantic search classification
    Babbage,

    /// Good at: Parsing text, simple classification, address correction, keywords
    Ada,
}

#[derive(Copy, Clone)]
pub enum TaskType {
    Completion,
    Search,
}

const API_BASE: &str = "https://api.openai.com/v1/engines";
impl From<&EngineType> for &'static str {
    fn from(e: &EngineType) -> Self {
        match e {
            EngineType::Davinci => "davinci",
            EngineType::Curie => "curie",
            EngineType::Babbage => "babbage",
            EngineType::Ada => "ada",
        }
    }
}

impl EngineType {
    fn to_endpoint(&self, task_type: TaskType) -> String {
        let task = match task_type {
            TaskType::Completion => "completions",
            TaskType::Search => "search",
        };
        let engine: &'static str = self.into();
        format!("{}/{}/{}", API_BASE, engine, task)
    }
}

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
            prompt: transform(text),
            max_tokens: 50,
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
            prompt: transform(text),
            max_tokens: 50,
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
            prompt: text,
            max_tokens: num_tokens,
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

use crate::gpt::domain::completion::{Options, CompletionResponse};

mod http;
mod domain;

/// Engine Types. Davinci is the most complex and expensive, Ada is the simplest and cheapest
enum EngineType {
    /// Good at: Complex intent, cause and effect, summarization for audience
    Davinci,

    /// Good at: Language translation, complex classification, text sentiment, summarization
    Curie,

    /// Good at: Moderate classification, semantic search classification
    Babbage,

    /// Good at: Parsing text, simple classification, address correction, keywords
    Ada,
}

enum TaskType {
    Completion,
    Search
}

const API_BASE: &str = "https://api.openai.com/v1/engines";
impl EngineType {
    fn to_str(&self) -> &'static str {
        match *self {
            EngineType::Davinci => "davinci",
            EngineType::Curie => "curie",
            EngineType::Babbage => "babbage",
            EngineType::Ada => "ada"
        }
    }

    fn to_endpoint(&self, task_type: TaskType) -> String {
        let task = match task_type {
            TaskType::Completion => "completions",
            TaskType::Search => "search",
        };
        format!("{}/{}/{}", API_BASE, self.to_str(), task)
    }
}

pub struct GPTClient {
    pub api_key: String,
}

impl GPTClient {
    #[tokio::main]
    pub async fn summarize(&self, text: String) -> String {
        const ENGINE: EngineType = EngineType::Curie;
        let options = Options {
            prompt: text,
            ..Default::default()
        };

        let res = http::rq(
            ENGINE.to_endpoint(TaskType::Completion),
            (&self.api_key).clone(),
            options,
        ).await;

        match res {
            Ok(v) => {
                let json = v.json::<CompletionResponse>().await.unwrap();
                serde_json::to_string(&json).unwrap()
            },
            Err(e) => format!("error making req: {:?}", e),
        }
    }

    fn elaborate(&self, text: String) -> String {
        "not implemented".parse().unwrap()
    }

    fn rephrase(&self, text: String) -> String {
        "not implemented".parse().unwrap()
    }

    fn complete(&self, text: String, num_tokens: u16) -> String {
        "not implemented".parse().unwrap()
    }

    // fn search(&self, document: String, query: String)
}
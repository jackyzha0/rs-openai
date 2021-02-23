use rocket::request::FromParam;
use rocket::http::RawStr;

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

impl<'r> FromParam<'r> for EngineType {
    type Error = &'r RawStr;
    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let p = param.as_str();
        match p {
            "davinci" => Ok(EngineType::Davinci),
            "curie" => Ok(EngineType::Curie),
            "babbage" => Ok(EngineType::Babbage),
            "ada" => Ok(EngineType::Ada),
            _ => Err(param)
        }
    }
}

impl EngineType {
    pub fn to_endpoint(&self, task_type: TaskType) -> String {
        let task = match task_type {
            TaskType::Completion => "completions",
            TaskType::Search => "search",
        };
        let engine: &'static str = self.into();
        format!("{}/{}/{}", API_BASE, engine, task)
    }
}

#[derive(Copy, Clone)]
pub enum TaskType {
    Completion,
    Search,
}

impl<'r> FromParam<'r> for TaskType {
    type Error = &'r RawStr;
    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let p = param.as_str();
        match p {
            "completions" => Ok(TaskType::Completion),
            "search" => Ok(TaskType::Search),
            _ => Err(param)
        }
    }
}
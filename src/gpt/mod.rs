mod domain;
mod http;

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

    fn to_endpoint(&self) -> String {
        format!("{base}/{engine}", base=API_BASE, engine=self.to_str())
    }
}

// fn for each type of task
// * summarization
// * elaboration
// * rephrase
// * search
// * completion
// just call function to make http request


// closure to grab env


// function to make http request
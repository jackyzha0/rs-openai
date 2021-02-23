

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    /// The prompt(s) to generate completions for, encoded as a string
    pub prompt: Option<String>,

    /// The maximum number of tokens to generate. Requests can use up to 2048 tokens shared between
    /// prompt and completion. (One token is roughly 4 characters for normal English text).
    pub max_tokens: Option<u16>,

    /// Higher values means the model will take more risks. Try 0.9 for more creative applications,
    /// and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// We generally recommend altering this or top_p but not both.
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model
    /// considers the results of the tokens with top_p probability mass. So 0.1 means only the
    /// tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or top_p but not both.
    pub top_p: Option<f32>,

    // How many completions to generate for each prompt.
    pub n: Option<u16>,

    /// Include the log probabilities on the logprobs n most likely tokens, as well the chosen
    /// tokens. For example, if logprobs is 10, the API will return a list of the 10 most likely
    /// tokens.
    pub logprobs: Option<u16>,

    /// Echo back the prompt in addition to the completion
    pub echo: Option<bool>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    /// The returned text will not contain the stop sequence.
    pub stop: Option<Vec<String>>,

    /// Number between 0 and 1 that penalizes new tokens based on whether they appear in the text
    /// so far. Increases the model's likelihood to talk about new topics.
    pub presence_penalty: Option<f32>,

    /// Number between 0 and 1 that penalizes new tokens based on their existing frequency in]
    /// the text so far. Decreases the model's likelihood to repeat the same line verbatim.
    pub frequency_penalty: Option<f32>,

    /// Generates best_of completions server-side and returns the "best" (the one with the lowest
    /// log probability per token).
    ///
    /// When used with n, best_of controls the number of candidate completions and n specifies how
    /// many to return – best_of must be greater than n.
    pub best_of: Option<u16>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            prompt: Some("<|endoftext|>".parse().unwrap()),
            max_tokens: Some(16),
            temperature: Some(1.0),
            top_p: Some(1.0),
            n: Some(1),
            logprobs: None,
            echo: Some(false),
            stop: None,
            presence_penalty: Some(0.0),
            frequency_penalty: Some(0.0),
            best_of: Some(1),
        }
    }
}

/// represents a response structure for completion API
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Completion>,
}

/// represents a single possible completion done by GPT-3
#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    text: String,
    index: u16,
    finish_reason: String,
    logprobs: Option<LogProbs>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogProbs {
    tokens: Vec<String>,
    token_logprobs: Vec<f32>,
    top_logprobs: HashMap<String, f32>,
    text_offset: Vec<u16>,
}

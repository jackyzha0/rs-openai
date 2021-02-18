struct Options {
    /// The prompt(s) to generate completions for, encoded as a string
    prompt: &'static str,

    /// The maximum number of tokens to generate. Requests can use up to 2048 tokens shared between
    /// prompt and completion. (One token is roughly 4 characters for normal English text).
    max_tokens: u16,

    /// Higher values means the model will take more risks. Try 0.9 for more creative applications,
    /// and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// We generally recommend altering this or top_p but not both.
    temperature: f32,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model
    /// considers the results of the tokens with top_p probability mass. So 0.1 means only the
    /// tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or top_p but not both.
    top_p: f32,

    // How many completions to generate for each prompt.
    n: u16,

    /// Include the log probabilities on the logprobs n most likely tokens, as well the chosen
    /// tokens. For example, if logprobs is 10, the API will return a list of the 10 most likely
    /// tokens.
    logprobs: Option<u16>,

    /// Echo back the prompt in addition to the completion
    echo: bool,

    /// Up to 4 sequences where the API will stop generating further tokens.
    /// The returned text will not contain the stop sequence.
    stop: Option<Vec<&'static str>>,

    /// Number between 0 and 1 that penalizes new tokens based on whether they appear in the text
    /// so far. Increases the model's likelihood to talk about new topics.
    presence_penalty: f32,

    /// Number between 0 and 1 that penalizes new tokens based on their existing frequency in]
    /// the text so far. Decreases the model's likelihood to repeat the same line verbatim.
    frequency_penalty: f32,

    /// Generates best_of completions server-side and returns the "best" (the one with the lowest
    /// log probability per token).
    ///
    /// When used with n, best_of controls the number of candidate completions and n specifies how
    /// many to return – best_of must be greater than n.
    best_of: u16,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            prompt: "<|endoftext|>",
            max_tokens: 16,
            temperature: 1.0,
            top_p: 1.0,
            n: 1,
            logprobs: None,
            echo: false,
            stop: None,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            best_of: 1
        }
    }
}

// this._construct_parameter("prompt", opts.prompt),
// this._construct_parameter("stream", opts.stream),
// this._construct_parameter("stop", opts.stop),
// this._construct_parameter("max_tokens", this._safe_cast(opts.maxTokens)),
// this._construct_parameter("temperature", this._safe_cast(opts.temperature)),
// this._construct_parameter("top_p", this._safe_cast(opts.topP)),
// this._construct_parameter("presence_penalty", this._safe_cast(opts.presencePenalty)),
// this._construct_parameter("frequency_penalty", this._safe_cast(opts.frequencyPenalty)),
// this._construct_parameter("best_of", this._safe_cast(opts.bestOf)),
// this._construct_parameter("n", this._safe_cast(opts.n)),
// this._construct_parameter("logprobs", this._safe_cast(opts.logprobs)),
// this._construct_parameter("echo", opts.echo),
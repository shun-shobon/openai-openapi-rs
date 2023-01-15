/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCompletionRequest {
    /// ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "prompt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Option<Box<crate::models::CreateCompletionRequestPrompt>>>,
    /// The suffix that comes after a completion of inserted text.
    #[serde(rename = "suffix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<Option<String>>,
    /// The maximum number of [tokens](/tokenizer) to generate in the completion.  The token count of your prompt plus `max_tokens` cannot exceed the model's context length. Most models have a context length of 2048 tokens (except for the newest models, which support 4096). 
    #[serde(rename = "max_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<Option<i32>>,
    /// What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.  We generally recommend altering this or `top_p` but not both. 
    #[serde(rename = "temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f32>>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or `temperature` but not both. 
    #[serde(rename = "top_p", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<Option<f32>>,
    /// How many completions to generate for each prompt.  **Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`. 
    #[serde(rename = "n", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub n: Option<Option<i32>>,
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message. 
    #[serde(rename = "stream", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stream: Option<Option<bool>>,
    /// Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.  The maximum value for `logprobs` is 5. If you need more than this, please contact us through our [Help center](https://help.openai.com) and describe your use case. 
    #[serde(rename = "logprobs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Option<i32>>,
    /// Echo back the prompt in addition to the completion 
    #[serde(rename = "echo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub echo: Option<Option<bool>>,
    #[serde(rename = "stop", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stop: Option<Option<Box<crate::models::CreateCompletionRequestStop>>>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.  [See more information about frequency and presence penalties.](/docs/api-reference/parameter-details) 
    #[serde(rename = "presence_penalty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<Option<f32>>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.  [See more information about frequency and presence penalties.](/docs/api-reference/parameter-details) 
    #[serde(rename = "frequency_penalty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<Option<f32>>,
    /// Generates `best_of` completions server-side and returns the \"best\" (the one with the highest log probability per token). Results cannot be streamed.  When used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.  **Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`. 
    #[serde(rename = "best_of", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub best_of: Option<Option<i32>>,
    /// Modify the likelihood of specified tokens appearing in the completion.  Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.  As an example, you can pass `{\"50256\": -100}` to prevent the <|endoftext|> token from being generated. 
    #[serde(rename = "logit_bias", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<Option<serde_json::Value>>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices/end-user-ids). 
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl CreateCompletionRequest {
    pub fn new(model: String) -> CreateCompletionRequest {
        CreateCompletionRequest {
            model,
            prompt: None,
            suffix: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            best_of: None,
            logit_bias: None,
            user: None,
        }
    }
}



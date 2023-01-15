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
pub struct CreateCompletionResponseUsage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i32,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i32,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i32,
}

impl CreateCompletionResponseUsage {
    pub fn new(prompt_tokens: i32, completion_tokens: i32, total_tokens: i32) -> CreateCompletionResponseUsage {
        CreateCompletionResponseUsage {
            prompt_tokens,
            completion_tokens,
            total_tokens,
        }
    }
}



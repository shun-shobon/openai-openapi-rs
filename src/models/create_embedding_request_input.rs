/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateEmbeddingRequestInput : Input text to get embeddings for, encoded as a string or array of tokens. To get embeddings for multiple inputs in a single request, pass an array of strings or array of token arrays. Each input must not exceed 8192 tokens in length.

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CreateEmbeddingRequestInput {}

impl CreateEmbeddingRequestInput {
    /// Input text to get embeddings for, encoded as a string or array of tokens. To get embeddings for multiple inputs in a single request, pass an array of strings or array of token arrays. Each input must not exceed 8192 tokens in length.
    pub fn new() -> CreateEmbeddingRequestInput {
        CreateEmbeddingRequestInput {}
    }
}

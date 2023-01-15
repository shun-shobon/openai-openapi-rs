/*
 * OpenAI API
 *
 * APIs for sampling from and fine-tuning language models
 *
 * The version of the OpenAPI document: 1.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DeleteModelResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl DeleteModelResponse {
    pub fn new(id: String, object: String, deleted: bool) -> DeleteModelResponse {
        DeleteModelResponse {
            id,
            object,
            deleted,
        }
    }
}

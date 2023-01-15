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
pub struct ImagesResponse {
    #[serde(rename = "created")]
    pub created: i32,
    #[serde(rename = "data")]
    pub data: Vec<crate::models::ImagesResponseDataInner>,
}

impl ImagesResponse {
    pub fn new(created: i32, data: Vec<crate::models::ImagesResponseDataInner>) -> ImagesResponse {
        ImagesResponse {
            created,
            data,
        }
    }
}


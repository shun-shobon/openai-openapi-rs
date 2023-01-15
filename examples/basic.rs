use openai_openapi::apis::configuration::{ApiKey, Configuration};
use openai_openapi::apis::open_ai_api;

#[tokio::main]
async fn main() {
    let config = Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: std::env::var("OPENAI_API_KEY").unwrap(),
        }),
        ..Default::default()
    };
    let models = open_ai_api::list_models(&config).await.unwrap();
    println!("{:?}", models);
}

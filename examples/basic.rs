use openai_openapi::apis::configuration::Configuration;
use openai_openapi::apis::open_ai_api;

#[tokio::main]
async fn main() {
    let config = Configuration {
        bearer_access_token: Some(
            std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string()),
        ),
        ..Default::default()
    };
    let models = open_ai_api::list_models(&config).await.unwrap();
    println!("{:?}", models);
}

use derive_more::{
    Constructor
};

/// Внутренняя структура билдера запросов с неизменяемыми данными
#[derive(Debug, Constructor)]
pub struct PocketApiConfig { 
    pub http_client: reqwest::Client, // Arc inside
    pub api_url: reqwest::Url,
    pub api_consumer_key: String
}

impl PocketApiConfig{
    pub fn new_default(http_client: reqwest::Client, api_consumer_key: String) -> PocketApiConfig {
        PocketApiConfig{
            http_client,
            api_consumer_key,
            api_url: reqwest::Url::parse("https://getpocket.com/").unwrap()
        }
    }
}
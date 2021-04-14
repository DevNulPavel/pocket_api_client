use std::{
    sync::{
        Arc
    }
};
use derive_more::{
    Constructor
};

#[derive(Debug, Constructor)]
pub struct Inner{
    pub http_client: reqwest::Client, // Arc inside
    pub api_url: reqwest::Url,
    pub api_consumer_key: String
}

/// Внутренняя структура билдера запросов с неизменяемыми данными
#[derive(Debug, Clone)]
pub struct PocketApiConfig { 
    inner: Arc<Inner>
}
impl PocketApiConfig{
    pub fn new_default(http_client: reqwest::Client, 
                       api_consumer_key: String) -> PocketApiConfig {
        let inner = Arc::new(Inner{
            http_client,
            api_consumer_key,
            api_url: reqwest::Url::parse("https://getpocket.com/").unwrap()
        });
        PocketApiConfig{
            inner
        }
    }
}
impl std::ops::Deref for PocketApiConfig{
    type Target = Inner;
    fn deref(&self) -> &Self::Target{
        &self.inner
    }
}
use std::{
    sync::{
        Arc
    }
};
use crate::{
    config::{
        PocketApiConfig
    },
    request_builder::{
        PocketRequestBuilder
    }
};

#[derive(Debug)]
pub struct PocketApiClient{
    request_builder: PocketRequestBuilder
}

impl PocketApiClient {
    pub fn new(config: Arc<PocketApiConfig>, user_token: String) -> PocketApiClient {
        let request_builder = PocketRequestBuilder::new(config)
            .user_api_token(user_token);
        PocketApiClient{
            request_builder
        }
    }

    pub async fn add(&self, url: String, title: Option<String>){
        
    }
}
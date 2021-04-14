use std::{
    collections::{
        HashMap
    }
};
use serde_json::{
    json,
    Value
};
use tracing::{
    instrument,
    debug
};
use crate::{
    helpers::{
        ReqwestExt
    },
    config::{
        PocketApiConfig
    },
    request_builder::{
        PocketRequestBuilder
    },
    error::{
        PocketApiError
    },
    responses::{
        NewItemInfoResponse,
        ItemsInfoResponse,
        ItemInfo,
        NewItemInfo
    }
};

#[derive(Debug)]
pub struct PocketApiClient{
    request_builder: PocketRequestBuilder
}

impl PocketApiClient {
    pub fn new(config: PocketApiConfig, user_token: String) -> PocketApiClient {
        let request_builder = PocketRequestBuilder::new(config)
            .user_api_token(user_token);
        PocketApiClient{
            request_builder
        }
    }

    #[instrument(skip(self))]
    pub async fn add(&self, url: String, title: Option<String>) -> Result<NewItemInfo, PocketApiError>{
        let mut data = json!({
            "url": url,
        });
        if let Some(title) = title {
            data["title"] = Value::String(title);
        }

        let req = self
            .request_builder
            .clone()
            .join_path("add".to_string())
            .json(data)
            .build()?;
        
        // Данные
        let resp = req
            .receive_json_checked::<NewItemInfoResponse>("Add item")
            .await?; 
        debug!("Received article info: {:#?}", resp);

        if resp.status != 1{
            return Err(PocketApiError::PocketInvalidApiStatus(resp.status));
        }

        Ok(resp.item)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self) -> Result<HashMap<String, ItemInfo>, PocketApiError>{
        let req = self
            .request_builder
            .clone()
            .join_path("get".to_string())
            .json(json!({
                "state": "all"
            }))
            .build()?;
        
        // Данные
        let resp = req
            .receive_json_checked::<ItemsInfoResponse>("Get all items")
            .await?; 
        debug!("Received article info: {:#?}", resp);

        if resp.status != 1{
            return Err(PocketApiError::PocketInvalidApiStatus(resp.status));
        }

        Ok(resp.list)
    }
}
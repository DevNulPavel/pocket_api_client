mod error;
mod responses;


use serde_json::{
    json
};
use quick_error::{
    ResultExt
};
use tracing::{
    instrument,
    debug
};
use async_trait::{
    async_trait
};
use serde::{
    Deserialize
};
use crate::{
    error::{
        PocketApiError
    },
    responses::{
        CodeRequestResponse
    }
};

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[async_trait]
pub trait ResponseExt {
    async fn json_with_debug<R: for<'de> Deserialize<'de>>(self, err_context: &'static str) -> Result<R, PocketApiError>;
}

#[async_trait]
impl ResponseExt for reqwest::Response {
    #[instrument]
    async fn json_with_debug<R: for<'de> Deserialize<'de>>(self, err_context: &'static str) -> Result<R, PocketApiError> {
        let full = self
            .bytes()
            .await
            .context(format!("Body receive failed with context: {}", err_context))?;

        debug!("Json data: {:?}", full);

        serde_json::from_slice::<R>(&full)
            .context(format!("Body json parse failed with context: {}", err_context))
            .map_err(PocketApiError::from)
    }
}

///////////////

#[async_trait]
pub trait ReqwestExt {
    async fn receive_json_checked<R: for<'de> Deserialize<'de>>(self, err_context: &'static str) -> Result<R, PocketApiError>;
}

#[async_trait]
impl ReqwestExt for reqwest::RequestBuilder {
    #[instrument]
    async fn receive_json_checked<R: for<'de> Deserialize<'de>>(self, err_context: &'static str) -> Result<R, PocketApiError>{
        let resp = self
            .send()
            .await
            .context(format!("Request failed with context: {}", err_context))?;

        if resp.status().is_success(){
            return resp
                .json_with_debug::<R>(err_context)
                .await;
        }
        let headers = resp.headers();
        let code = headers.get("X-Error-Code")
            .and_then(|v| {
                v.to_str().ok()
            })
            .map(|v| {
                v.to_string()
            });
        let desc = headers.get("X-Error")
            .and_then(|v| {
                v.to_str().ok()
            })
            .map(|v| {
                v.to_string()
            });
        match (code, desc) {
            (Some(code), Some(desc)) => {
                return Err(PocketApiError::PocketApiError(resp.status(), code, desc))
            },
            _ => {
                return Err(PocketApiError::PocketUnknownApiError(resp.status()))
            }
        }
    }
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct PocketApiClient{
    http_client: reqwest::Client,
    base_url: reqwest::Url,
    consumer_api_key: String
}
impl PocketApiClient {
    pub fn new(http_client: reqwest::Client, 
               base_url: reqwest::Url,
               consumer_api_key: String) -> PocketApiClient {
        PocketApiClient{
            http_client,
            base_url,
            consumer_api_key
        }
    }

    #[instrument]
    pub async fn optain_request_token(&self, redirect_uri: String) -> Result<String, PocketApiError>{
        let mut url = self.base_url.clone();
        {
            url
                .path_segments_mut()
                .map_err(|_|{
                    PocketApiError::InvalidBaseUrl
                })?
                .push("oauth")
                .push("request");
        }
        
        let resp = self
            .http_client
            .post(url)
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("X-Accept", "application/json")
            .json(&json!(
                {
                    "consumer_key": self.consumer_api_key,
                    "redirect_uri": redirect_uri
                }
            ))
            .receive_json_checked::<CodeRequestResponse>("Token obtain error")
            .await?; 
            
        Ok(resp.code)
    }
}
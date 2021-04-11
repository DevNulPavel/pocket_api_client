use quick_error::{
    ResultExt
};
use tracing::{
    instrument,
    // debug
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
    helpers::{
        response_ext::{
            ResponseExt
        }
    }
};

#[async_trait]
pub trait ReqwestExt {
    async fn receive_json_checked<R: for<'de> Deserialize<'de>>(self, err_context: &'static str) -> Result<R, PocketApiError>;
}

#[async_trait]
impl ReqwestExt for reqwest::RequestBuilder {
    #[instrument(skip(self, err_context))]
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
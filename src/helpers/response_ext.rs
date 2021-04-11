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
    }
};

#[async_trait]
pub trait ResponseExt {
    async fn json_with_debug<R: for<'de> Deserialize<'de>>(self, err_context: &'static str) -> Result<R, PocketApiError>;
}

#[async_trait]
impl ResponseExt for reqwest::Response {
    #[instrument(skip(self, err_context))]
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
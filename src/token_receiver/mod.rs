use std::{
    sync::{
        Arc
    }
};
use serde_json::{
    json
};
use tracing::{
    instrument,
    debug
};
use derive_more::{
    Display
};
use crate::{
    error::{
        PocketApiError
    },
    config::{
        PocketApiConfig
    },
    responses::{
        CodeRequestResponse,
        TokenRequestResponse
    },
    helpers::{
        ReqwestExt
    },
    request_builder::{
        PocketRequestBuilder
    }
};

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Display)]
#[display(fmt = "code = {}, auth_url = {}", code, auth_url)]
pub struct AuthInfo{
    pub code: String,
    pub auth_url: reqwest::Url
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct PocketApiTokenReceiver{
    request_builder: PocketRequestBuilder
}

impl PocketApiTokenReceiver {
    pub fn new(config: Arc<PocketApiConfig>) -> PocketApiTokenReceiver {
        let request_builder = PocketRequestBuilder::new(config);
        PocketApiTokenReceiver{
            request_builder
        }
    }

    /// Данный метод выдает url для подтверждения пользователем разрешений на использование приложения
    /// После этого уже можно полноценно получать токен
    #[instrument(skip(self))]
    pub async fn optain_user_auth_info(&self, redirect_uri: String) -> Result<AuthInfo, PocketApiError>{
        let req = self
            .request_builder
            .clone()
            .join_path("oauth".to_string())
            .join_path("request".to_string())
            .json(json!({
                "redirect_uri": redirect_uri
            }))
            .build()?;
        
        // Получаем код для авторизации пользователя для формирования ссылки ниже
        let resp = req
            .receive_json_checked::<CodeRequestResponse>("Token obtain error")
            .await?; 
        debug!("Received code: {}", resp.code);
            
        // Ссылка, куда должен перейти пользователь для подтверждения разрешения
        let mut auth_url = reqwest::Url::parse("https://getpocket.com/auth/authorize")
            .unwrap();
        auth_url
            .query_pairs_mut()
            .append_pair("request_token", &resp.code)
            .append_pair("redirect_uri", &redirect_uri);
        debug!("Auth url: {}", auth_url);

        Ok(AuthInfo{
            auth_url,
            code: resp.code
        })
    }

    /// Метод получения токена после подтверждения прав
    #[instrument(skip(self))]
    pub async fn receive_token(&self, code: String) -> Result<String, PocketApiError>{
        let req = self
            .request_builder
            .clone()
            .join_path("oauth".to_string())
            .join_path("authorize".to_string())
            .json(json!({
                "code": code
            }))
            .build()?;
        
        // Получаем код для авторизации пользователя для формирования ссылки ниже
        let resp = req
            .receive_json_checked::<TokenRequestResponse>("Token obtain error")
            .await?; 
        debug!("Received token: {}", resp);

        Ok(resp.access_token)
    }
}

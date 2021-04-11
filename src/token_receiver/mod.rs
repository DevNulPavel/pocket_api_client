
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
    responses::{
        CodeRequestResponse,
        TokenRequestResponse
    },
    helpers::{
        ReqwestExt
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
    http_client: reqwest::Client,
    base_url: reqwest::Url,
    consumer_api_key: String
}

impl PocketApiTokenReceiver {
    pub fn new(http_client: reqwest::Client, 
               consumer_api_key: String) -> PocketApiTokenReceiver {
        PocketApiTokenReceiver{
            http_client,
            base_url: reqwest::Url::parse("https://getpocket.com/").unwrap(),
            consumer_api_key
        }
    }

    pub fn new_with_base_url(http_client: reqwest::Client, 
                             base_url: reqwest::Url,
                             consumer_api_key: String) -> PocketApiTokenReceiver {
        PocketApiTokenReceiver{
            http_client,
            base_url,
            consumer_api_key
        }
    }

    /// Данный метод выдает url для подтверждения пользователем разрешений на использование приложения
    /// После этого уже можно полноценно получать токен
    #[instrument(skip(self))]
    pub async fn optain_user_auth_info(&self, redirect_uri: String) -> Result<AuthInfo, PocketApiError>{
        let mut url = self.base_url.clone();
        {
            url
                .path_segments_mut()
                .map_err(|_|{
                    PocketApiError::InvalidBaseUrl
                })?
                .push("v3")
                .push("oauth")
                .push("request");
        }
        debug!("Request url: {}", url);
        
        // Получаем код для авторизации пользователя для формирования ссылки ниже
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
        let mut url = self.base_url.clone();
        {
            url
                .path_segments_mut()
                .map_err(|_|{
                    PocketApiError::InvalidBaseUrl
                })?
                .push("v3")
                .push("oauth")
                .push("authorize");
        }
        debug!("Request url: {}", url);
        
        // Получаем код для авторизации пользователя для формирования ссылки ниже
        let resp = self
            .http_client
            .post(url)
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("X-Accept", "application/json")
            .json(&json!(
                {
                    "consumer_key": self.consumer_api_key,
                    "code": code
                }
            ))
            .receive_json_checked::<TokenRequestResponse>("Token obtain error")
            .await?; 
        debug!("Received token: {}", resp);

        Ok(resp.access_token)
    }
}


// TODO: Mock testing
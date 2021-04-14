use std::{
    ops::{
        Deref
    }
};
use cow_arc::{
    CowArc
};
use crate::{
    error::{
        PocketApiError
    },
    config::{
        PocketApiConfig
    }
};

/// Непосредственно реквест билдер, который может быть легко склонирован с тем состоянием,
/// которое он имеет сейчас на момент работы
#[derive(Debug, Clone)]
pub struct PocketRequestBuilder {
    base: PocketApiConfig,
    api_token: CowArc<Option<String>>,
    path_segments: CowArc<Vec<String>>,
    json: CowArc<Option<serde_json::Value>>,
}
impl<'a> PocketRequestBuilder {
    pub fn new(config: PocketApiConfig) -> PocketRequestBuilder {

        PocketRequestBuilder{
            base: config,
            api_token: Default::default(),
            path_segments: Default::default(),
            json: Default::default()
        }
    }

    /// Возвращает сырой клиент без модификаций
    /*pub fn get_http_client(&self) -> Client {
        self.base.http_client.clone()
    }*/

    pub fn join_path(mut self, segment: String) -> PocketRequestBuilder {
        self.path_segments.update_val(|val|{
            val.push(segment);
        });
        self
    }

    /*pub fn join_paths(mut self, segments: Vec<String>) -> RequestBuilder {
        self.path_segments.update_val(|val|{
            segments
                .into_iter()
                .for_each(|segment|{
                    val.push(segment);
                })
        });
        self
    }*/

    pub fn json(mut self, data: serde_json::Value) -> PocketRequestBuilder {
        self.json.set_val(Some(data));
        self
    }

    pub fn user_api_token(mut self, token: String) -> PocketRequestBuilder {
        self.api_token.set_val(Some(token));
        self
    }

    pub fn build(self) -> Result<reqwest::RequestBuilder, PocketApiError>{
        let mut url = self.base.api_url.clone();
        {
            let mut segments = url.path_segments_mut()
                .map_err(|_|{
                    PocketApiError::InvalidBaseUrl
                })?;

            // Базовая часть
            segments.push("v3");

            // Сегменты
            for segment in self.path_segments.deref() {
                let segment = segment.trim_matches('/');
                let split = segment.split("/");
                for part in split{
                    segments.push(part);
                }
            }
        }

        // Json
        let mut json = self
            .json
            .as_ref()
            .map(|v| {
                v.to_owned()
            })
            .unwrap_or_else(|| {
                serde_json::json!({})
            });
    
        // Consumer key
        json["consumer_key"] = serde_json::value::Value::String(self.base.api_consumer_key.clone());

        // Api token
        if let Some(api_token) = self.api_token.as_ref() {
            json["access_token"] = serde_json::value::Value::String(api_token.to_owned());
        }

        // Создаем запрос
        let builder = self.base.http_client
            .post(url)
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("X-Accept", "application/json")
            .json(&json);

        Ok(builder)
    }
}
use std::{
    collections::{
        HashMap
    }
};
use serde_json::{
    Value
};
use derive_more::{
    Display
};
use serde::{
    Deserialize
};

//////////////////////////////////////////////////////////////////////

/*/// Тип ошибки, в который мы можем парсить наши данные
#[derive(Deserialize, Debug)]
pub struct ErrorResponseValue{
    #[serde(flatten)]
    other: HashMap<String, Value>
}

//////////////////////////////////////////////////////////////////////

/// Специальный шаблонный тип, чтобы можно было парсить возвращаемые ошибки в ответах
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum DataOrErrorResponse<D>{
    Ok(D),
    Err(ErrorResponseValue)
}
impl<D> DataOrErrorResponse<D> {
    pub fn into_result(self) -> Result<D, ErrorResponseValue> {
        match self {
            DataOrErrorResponse::Ok(ok) => Ok(ok),
            DataOrErrorResponse::Err(err) => Err(err),
        }
    }
}*/

//////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Debug, Display)]
pub struct CodeRequestResponse{
    pub code: String
}

//////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Debug, Display)]
#[display(fmt = "access_token = {}, username = {}", access_token, username)]
pub struct TokenRequestResponse{
    pub access_token: String,
    pub username: String,
}

//////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Debug, Display)]
#[display(fmt = "id = {}, title = {}, url = {}", item_id, title, normal_url)]
pub struct NewItemInfo{
    pub item_id: String,
    pub normal_url: String,
    pub title: String,

    #[serde(flatten)]
    pub other: HashMap<String, Value>
}

#[derive(Deserialize, Debug, Display)]
#[display(fmt = "status: {}, item: ({})", status, item)]
pub struct NewItemInfoResponse{
    pub status: i64,
    pub item: NewItemInfo
}

//////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Debug, Display)]
#[display(fmt = "id = {}, title = {}, url = {}", item_id, resolved_title, resolved_url)]
pub struct ItemInfo{
    pub item_id: String,
    pub resolved_url: String,
    pub resolved_title: String,

    #[serde(flatten)]
    pub other: HashMap<String, Value>
}

#[derive(Deserialize, Debug, Display)]
#[display(fmt = "status: {}, items_count: {}", "status", "list.len()")]
pub struct ItemsInfoResponse{
    pub status: i64,
    pub list: HashMap<String, ItemInfo>
}
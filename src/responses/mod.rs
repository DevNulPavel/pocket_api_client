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
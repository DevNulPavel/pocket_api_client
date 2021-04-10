use quick_error::{
    quick_error
};


quick_error!{
    #[derive(Debug)]
    pub enum PocketApiError{
        InvalidBaseUrl{
        }

        RequestError(context: String, err: reqwest::Error){
            context(context: &'static str, err: reqwest::Error) -> (context.to_string(), err)
            context(context: String, err: reqwest::Error) -> (context, err)
        }

        JsonParseError(context: String, err: serde_json::Error){
            context(context: &'static str, err: serde_json::Error) -> (context.to_string(), err)
            context(context: String, err: serde_json::Error) -> (context, err)
        }

        PocketApiError(http_status: reqwest::StatusCode, err_code: String, err_desc: String){
        }

        PocketUnknownApiError(http_status: reqwest::StatusCode){
        }
    }
}
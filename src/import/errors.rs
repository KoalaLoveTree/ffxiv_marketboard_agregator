use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("Error while sending request: {message:?}")]
    RequestError { message: String },
    #[error("Error while parsing response: {message:?}")]
    ResponseParseError { message: String },
    #[error("Error while parsing json: {message:?}")]
    JsonParseError { message: String },
    #[error("Json does not have field: {field_name:?}")]
    JsonMissedDataError { field_name: String },
    #[error("Error while parsing url string: {message:?}")]
    UrlParseError { message: String },
    #[error("Error while parsing url string: {message:?}")]
    SqlExecutionError { message: String },
    #[error("Error while accessing HashMap element")]
    HashMapAccessError,
    #[error("URL is cannot-be-a-base")]
    WrongUrlBaseError,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{message:?}")]
    ReqwestError { message: String },
    #[error("{message:?}")]
    JsonError { message: String },
    #[error("Json does not have field: {field_name:?}")]
    JsonMissedDataError { field_name: String },
    #[error("Error while parsing url string")]
    UrlParseError,
    #[error("Error while accessing HashMap element")]
    HashMapAccessError,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError {
            message: err.to_string(),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::JsonError {
            message: err.to_string(),
        }
    }
}
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::JsonError {
            message: err.to_string(),
        }
    }
}

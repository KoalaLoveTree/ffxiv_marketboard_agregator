use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wrong base for URL!")]
    UrlParseBase,
    #[error("Error while processing response")]
    Reqwest(#[from] reqwest::Error),
    #[error("Error while processing response")]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    #[error("Error while processing response Json")]
    Json(#[from] serde_json::Error),
    #[error("Error while parsing url string")]
    UrlParse(#[from] url::ParseError),
    #[error("{message}")]
    StringError { message: String },
}

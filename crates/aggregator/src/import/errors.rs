use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while accessing HashMap element")]
    HashMapAccess,
    #[error("Error while working with Universalis SDK")]
    UniversalisApiSDK(#[from] universalis_sdk::errors::Error),
    #[error("Error while processing tasks in parallel")]
    TokioJoin(#[from] JoinError),
    #[error("Error while executing sql")]
    Sqlx(#[from] sqlx::Error),
    #[error("Error while accessing HashMap element")]
    Test,
}

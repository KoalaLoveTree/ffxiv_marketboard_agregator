use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while executing sql")]
    Sqlx(#[from] sqlx::Error),
}

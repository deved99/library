use thiserror;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error.")]
    Database(#[from] sqlx::Error),
    #[error("Serde error.")]
    Serde(#[from] serde_json::Error),
    #[error("IO error.")]
    IO(#[from] std::io::Error),
    #[error("Unexpected number of results: expected {expected}, found: \n{results}")]
    UnexpectedResultNumber { expected: usize, results: String },
    #[error("Missing some arguments: expected {expected}")]
    MissingArgument { expected: String }
}

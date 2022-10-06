use thiserror;

pub type Result<T> = std::result::Result<T,Error>;
#[derive(thiserror::Error,Debug)]
pub enum Error {
    #[error("Database error.")]
    Database(#[from] sqlx::Error)
}

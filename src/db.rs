// mod book;
// mod author;
// mod tag;

use dotenv_codegen::dotenv;
use thiserror;
// Used to create a database pool
use once_cell::sync::OnceCell;
// Used to connect to the database
use sqlx::{
    self,
    postgres::{PgPool, PgPoolOptions},
};

static DATABASE_POOL: OnceCell<PgPool> = OnceCell::new();
pub async fn get_pool() -> &'static PgPool {
    if DATABASE_POOL.get().is_none() {
        log::debug!("Creating database pool.");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(dotenv!("DATABASE_URL"))
            .await.unwrap();
        DATABASE_POOL.set(pool).unwrap();
        log::debug!("Created database pool.");
    }
    DATABASE_POOL.get().unwrap()
}


type Result<T> = std::result::Result<T,Error>;
#[derive(thiserror::Error,Debug)]
pub enum Error {
    #[error("Database error.")]
    Database(#[from] sqlx::Error)
}

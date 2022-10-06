mod book;
mod author;
mod tag;

// Used to create a database pool
use once_cell::sync::OnceCell;
// Used to connect to the database
use sqlx::{
    self,
    postgres::{PgPool, PgPoolOptions},
};

static DATABASE_POOL: OnceCell<MySqlPool> = OnceCell::new();
pub async fn get_pool() -> &'static MySqlPool {
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

use dotenv_codegen::dotenv;
// Used to create a database pool
use once_cell::sync::OnceCell;
// Used to connect to the database
use sqlx::{
    self,
    postgres::{PgPool, PgPoolOptions},
};

mod as_row;
mod author;
mod book;
mod dump;
mod links;
pub mod schema;
mod tag;
mod views;

// Re export
use crate::Result;
pub use as_row::{print_table, AsRow};
pub use author::Author;
pub use book::{Book, BookDump};
pub use dump::Dump;
pub use links::{AuthorBook, TagBook};
pub use tag::Tag;
pub use views::ReadingList;

//// Functions

static DATABASE_POOL: OnceCell<PgPool> = OnceCell::new();
pub async fn get_pool() -> Result<&'static PgPool> {
    if DATABASE_POOL.get().is_none() {
        log::debug!("Creating database pool.");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(dotenv!("DATABASE_URL"))
            .await?;
        DATABASE_POOL.set(pool).unwrap();
        log::debug!("Created database pool.");
    }
    Ok(DATABASE_POOL.get().unwrap())
}

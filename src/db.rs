use dotenv_codegen::dotenv;
// Used to create a database pool
use once_cell::sync::OnceCell;
// Pretty print of tables
use prettytable;
// Used to connect to the database
use sqlx::{
    self,
    postgres::{PgPool, PgPoolOptions},
};

mod author;
mod book;
mod links;
mod reading_list;
mod tag;

// Re export
use crate::{Error, Result};
pub use author::Author;
pub use book::Book;
pub use links::{AuthorBook, TagBook};
pub use reading_list::ReadingList;
pub use tag::Tag;

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

pub trait AsRow {
    fn titles() -> Vec<String>;
    fn columns(&self) -> Vec<String>;
}

pub fn print_table<T: AsRow>(rows: &Vec<T>) {
    let mut table = prettytable::Table::new();
    // Title
    let titles = T::titles();
    table.add_row(prettytable::Row::from(&titles));
    // Then add each row
    for row in rows {
        let columns = row.columns();
        table.add_row(prettytable::Row::from(columns));
    }
    // Finally print
    table.printstd();
}

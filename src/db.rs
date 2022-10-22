use dotenv_codegen::dotenv;
// Used to create a database pool
use once_cell::sync::OnceCell;
// Pretty print of tables
use prettytable;
use serde::Serialize;
use serde_json;
// Used to connect to the database
use sqlx::{
    self,
    postgres::{PgPool, PgPoolOptions},
};

mod author;
mod book;
mod dump;
mod links;
mod tag;

// Re export
use crate::config;
use crate::Result;
pub use author::Author;
pub use book::{Book, BookDump};
pub use dump::Dump;
pub use links::{AuthorBook, TagBook};
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

pub fn print_table<T: Serialize + AsRow>(rows: &[T]) -> Result<()> {
    let config = config::get_config()?;
    let if_pretty = !config.json;
    match if_pretty {
        true => print_table_asrow(rows),
        false => print_table_serde(rows),
    }
}

fn print_table_serde<T: Serialize>(rows: &[T]) -> Result<()> {
    let lines = serde_json::to_string(rows)?;
    println!("{}", lines);
    Ok(())
}

fn print_table_asrow<T: AsRow>(rows: &[T]) -> Result<()> {
    let mut table = prettytable::Table::new();
    let format = *prettytable::format::consts::FORMAT_BOX_CHARS;
    table.set_format(format);
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
    Ok(())
}

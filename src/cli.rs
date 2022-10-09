use crate::actions;
use crate::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// List books in the reading list
    List,
    /// Insert a book in the database
    Insert { title: String, year: i16 },
}
impl Command {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book_list().await,
            Self::Insert { title, year } => actions::book_insert(&title, year).await,
        }
    }
}

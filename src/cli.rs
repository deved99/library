use std::path::PathBuf;

use crate::actions;
use crate::Result;

use clap::{Parser, Subcommand};

mod author;
mod book;
mod tag;

#[derive(Parser)]
pub struct Cli {
    /// Print output in json format
    #[arg(long)]
    pub json: bool,

    /// Don't ask for interaction
    #[arg(long)]
    pub unattended: bool,

    /// Increase output verbosity
    #[arg(short, long)]
    pub verbose: bool,

    /// Config file path
    #[arg(long)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// List books in the reading list
    Show,

    /// Actions related to a book
    #[command(subcommand)]
    Book(book::Book),

    /// Actions related to an author
    #[command(subcommand)]
    Author(author::Author),

    /// Actions related to a tag
    #[command(subcommand)]
    Tag(tag::Tag),

    /// Import a json file
    Import{
        /// Path to the file containing the data to import
        path: PathBuf
    },

    /// Export a json file
    Export{
        /// Path to the file where dumps will be saved; if none print to stdout
        path: Option<PathBuf>
    },

    /// Reset/init the database schema
    Reset{
        /// I'm sure, I really am sure
        #[arg(long)]
        force: bool
    },

    /// Currently testing
    Test,
}
impl Command {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Show => actions::views::reading_list().await,
            Self::Book(b) => b.execute().await,
            Self::Author(a) => a.execute().await,
            Self::Tag(t) => t.execute().await,
            Self::Import { path } => actions::dump::import(path).await,
            Self::Export { path } => actions::dump::export(path).await,
            Self::Reset { force } => actions::schema::reset(force).await,
            Self::Test => test(),
        }
    }
}

use crate::miscutils;
fn test() -> Result<()> {
    let s = miscutils::confirm(true)?;
    println!("You wrote {:?}", s);
    Ok(())
}

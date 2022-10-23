use std::path::PathBuf;

use crate::actions;
use crate::Result;

use clap::{Parser, Subcommand};

mod book;
mod author;
mod tag;
mod dump;

#[derive(Parser)]
pub struct Cli {
    /// Print output in json format
    #[arg(long)]
    pub json: bool,

    /// Don't ask for interaction (default if stdin != tty)
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
    List,

    /// Actions related to a book
    #[command(subcommand)]
    Book(book::Book),

    /// Actions related to an author
    #[command(subcommand)]
    Author(author::Author),

    /// Actions related to a tag
    #[command(subcommand)]
    Tag(tag::Tag),

    /// Import, export a Dump
    #[command(subcommand)]
    Dump(dump::Dump),

    Test,
}
impl Command {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book::list().await,
            Self::Book(b) => b.execute().await,
            Self::Author(a) => a.execute().await,
            Self::Tag(t) => t.execute().await,
            Self::Dump(d) => d.execute().await,
            Self::Test => test(),
        }
    }
}

use crate::miscutils;
fn test() -> Result<()> {
    let s = miscutils::confirm()?;
    println!("You wrote {:?}", s);
    Ok(())
}

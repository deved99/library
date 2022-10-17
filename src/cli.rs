use std::path::PathBuf;

use crate::actions;
use crate::Result;

use clap::{Args, Parser, Subcommand};

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
    Book(Book),

    /// Actions related to an author
    #[command(subcommand)]
    Author(Author),

    /// Actions related to a tag
    #[command(subcommand)]
    Tag(Tag),

    /// Import, export a Dump
    #[command(subcommand)]
    Dump(Dump),
}
impl Command {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book::list().await,
            Self::Book(b) => b.execute().await,
            Self::Author(a) => a.execute().await,
            Self::Tag(t) => t.execute().await,
            Self::Dump(d) => d.execute().await,
        }
    }
}

#[derive(Subcommand)]
pub enum Book {
    /// List books in the reading list
    List,
    /// Insert a book in the database
    Insert {
        #[arg(long)]
        title: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        year: i16,
        #[arg(long)]
        tag: Vec<String>,
    },
}
impl Book {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book::list().await,
            Self::Insert {
                title,
                year,
                author,
                tag,
            } => actions::book::insert(&title, &author, year, &tag).await,
        }
    }
}

#[derive(Subcommand)]
pub enum Author {
    /// List books in the reading list
    List,
    /// Insert a book in the database
    Insert {
        #[arg(long)]
        name: String,
        #[arg(long)]
        lang: String,
    },
}
impl Author {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::author::list().await,
            Self::Insert { name, lang } => actions::author::insert(&name, &lang).await,
        }
    }
}

#[derive(Subcommand)]
pub enum Tag {
    List,
}
impl Tag {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::tag::list().await,
        }
    }
}

#[derive(Subcommand)]
pub enum Dump {
    Import{
        /// File to import
        path: String
    },
    Export
}

impl Dump {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Import{path} => actions::dump::import(path).await,
            Self::Export => actions::dump::export().await
        }
    }
}

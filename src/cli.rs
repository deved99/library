use crate::actions;
use crate::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// List books in the reading list
    List,
    /// Book related actions
    #[command(subcommand)]
    Book(Book),
    /// Author related actions
    #[command(subcommand)]
    Author(Author),
}
impl Command {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book_list().await,
            Self::Book(b) => b.execute().await,
            Self::Author(a) => a.execute().await,
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
    },
}
impl Book {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book_list().await,
            Self::Insert {
                title,
                year,
                author,
            } => actions::book_insert(&title, &author, year).await,
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
            Self::List => actions::book_list().await,
            Self::Insert { name, lang } => actions::author_insert(&name, &lang).await,
        }
    }
}

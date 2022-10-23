use crate::{actions, Result};
use clap::Subcommand;
use chrono;
use uuid::Uuid;

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
    Delete {
        uuid: Uuid,
    },
    /// Start a book
    Start {
        #[arg(long)]
        uuid: Uuid,
        #[arg(long)]
        date: Option<chrono::NaiveDate>,
    },
    /// Finish a book
    Finish {
        #[arg(long)]
        uuid: Uuid,
        #[arg(long)]
        date: Option<chrono::NaiveDate>,
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
            Self::Delete { uuid } => actions::book::delete(uuid).await,
            Self::Start { uuid, date } => actions::book::start(uuid, date).await,
            Self::Finish { uuid, date } => actions::book::finish(uuid, date).await,
        }
    }
}

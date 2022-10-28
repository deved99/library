use crate::{actions, Result};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Author {
    /// List books in the reading list
    List,
    /// Insert a book in the database
    Insert {
        #[arg(long)]
        name: String,
    },
}

impl Author {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::author::list().await,
            Self::Insert { name } => actions::author::insert(&name).await,
        }
    }
}

use crate::{actions, Result};
use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum Author {
    /// List books in the reading list
    List,
    /// Insert a book in the database
    Insert {
        name: String,
    },
    Update {
        uuid: Uuid,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        order_name: Option<String>,
    },
    Delete {
        uuid: Uuid,
    },
}

impl Author {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::author::list().await,
            Self::Insert { name } => actions::author::insert(&name).await,
            Self::Update {
                uuid,
                display_name,
                order_name,
            } => actions::author::update(uuid, display_name, order_name).await,
            Self::Delete { uuid } => actions::author::delete(uuid).await,
        }
    }
}

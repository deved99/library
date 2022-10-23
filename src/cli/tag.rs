use crate::{actions, Result};
use clap::Subcommand;

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

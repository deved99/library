use crate::{actions, Result};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Dump {
    Import {
        /// File to import
        path: String,
    },
    Export,
}

impl Dump {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Import { path } => actions::dump::import(path).await,
            Self::Export => actions::dump::export().await,
        }
    }
}

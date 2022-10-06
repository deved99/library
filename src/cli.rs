use super::actions;
use super::Result;
use clap::{Subcommand, Parser};

#[derive(Parser)]
pub struct Cli {
   #[command(subcommand)]
   pub command: Command 
}

#[derive(Subcommand)]
pub enum Command {
    /// List books in the reading list
    List
}
impl Command {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book_list(),
        }.await
    }
}

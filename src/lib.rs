
mod actions;
mod error_handling;
mod cli;
mod db;

use clap::Parser;
use cli::Cli;

// Re-exports
pub use error_handling::{Error,Result};

pub async fn main() {
    let args = Cli::parse();
    args.command.execute().await.unwrap();
}

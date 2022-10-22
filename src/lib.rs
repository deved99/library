mod cli;
mod config;
mod db;
mod error_handling;
mod miscutils;

mod actions {
    pub mod author;
    pub mod book;
    pub mod dump;
    pub mod tag;
}

use clap::Parser;
use cli::Cli;

// Re-exports
pub use error_handling::{Error, Result};

pub async fn main() -> Result<()> {
    // Parse arguments
    let args = Cli::parse();

    // Then load configuration
    config::load_config(&args).unwrap();

    // Finally do what you've been asked
    let res = args.command.execute().await;
    if let Err(ref e) = res {
        eprintln!("{}\n", e);
    }
    res
}

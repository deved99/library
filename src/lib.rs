mod cli;
mod config;
mod db;
mod error_handling;

mod actions {
    pub mod author;
    pub mod book;
    pub mod tag;
}

use clap::Parser;
use cli::Cli;

// Re-exports
pub use error_handling::{Error, Result};

pub async fn main() {
    // Parse arguments
    let args = Cli::parse();

    // Then load configuration
    config::load_config(&args).unwrap();

    // Finally do what you've been asked
    let res = args.command.execute().await;
    if let Err(ref e) = res {
        println!("{}\n", e);
        res.unwrap();
    }
}

mod actions;
mod cli;
mod db;
mod error_handling;

use clap::Parser;
use cli::Cli;

// Re-exports
pub use error_handling::{Error, Result};

pub async fn main() {
    let args = Cli::parse();
    let res = args.command.execute().await;
    if let Err(ref e) = res {
        println!("{}\n", e);
        res.unwrap();
    }
}

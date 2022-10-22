use crate::db;
use crate::Result;
use serde_json;
use std::fs::File;
use std::path::Path;

pub async fn import(path: impl AsRef<Path>) -> Result<()> {
    let reader = File::open(path)?;
    let dumps: db::Dump = serde_json::from_reader(reader)?;
    dumps.import().await
}

pub async fn export() -> Result<()> {
    let dump = db::Dump::export().await?;
    let json = serde_json::to_string(&dump)?;
    println!("{}", json);
    Ok(())
}

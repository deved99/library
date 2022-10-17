use std::fs::File;
use std::path::Path;
use crate::Result;
use crate::db;
use serde_json;

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

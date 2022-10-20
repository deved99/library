use crate::db;
use crate::{Error, Result};

pub async fn list() -> Result<()> {
    let authors = db::Author::list().await?;
    db::print_table(&authors);
    Ok(())
}

pub async fn insert(name: &str, lang: &str) -> Result<()> {
    let book = db::Author::new(name, lang).await?;
    println!("{:?}", book);
    Ok(())
}

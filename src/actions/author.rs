use crate::db;
use crate::Result;

pub async fn list() -> Result<()> {
    let authors = db::Author::list().await?;
    db::print_table(&authors)?;
    Ok(())
}

pub async fn insert(name: &str) -> Result<()> {
    let book = db::Author::new(name).await?;
    println!("{:?}", book);
    Ok(())
}

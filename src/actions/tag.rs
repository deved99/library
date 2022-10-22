use crate::db;
use crate::Result;

pub async fn list() -> Result<()> {
    let tags = db::Tag::list().await?;
    db::print_table(&tags)?;
    Ok(())
}

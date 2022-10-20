use crate::db;
use crate::{Error, Result};

pub async fn list() -> Result<()> {
    let tags = db::Tag::list().await?;
    db::print_table(&tags);
    Ok(())
}

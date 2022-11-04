use crate::{db, Result};

pub async fn reading_list() -> Result<()> {
    let books = db::ReadingList::get().await?;
    db::print_table(&books)?;
    Ok(())
}

use super::{Result,Error};
use super::db;

pub async fn book_list() -> Result<()> {
    let books = db::Book::list().await?;
    for book in books {
        println!("{:?}", book)
    }
    Ok(())
}

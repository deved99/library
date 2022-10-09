use crate::db;
use crate::{Error, Result};

pub async fn book_list() -> Result<()> {
    let books = db::Book::list().await?;
    for book in books {
        println!("{:?}", book)
    }
    Ok(())
}

pub async fn book_insert(title: &str, year: i16) -> Result<()> {
    let book = db::Book::new(title, year).await?;
    println!("{:?}", book);
    Ok(())
}

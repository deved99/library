use crate::db;
use crate::{Error, Result};

pub async fn book_list() -> Result<()> {
    let books = db::Book::list().await?;
    for book in books {
        println!("{:?}", book)
    }
    Ok(())
}

pub async fn book_insert(title: &str, author: &str, year: i16) -> Result<()> {
    let artists = db::Author::find(author).await?;
    let author = match artists.len() {
        1 => &artists[0],
        _ => {
            return Err(Error::UnexpectedResultNumber {
                expected: 1,
                results: format!("{:?}", artists),
            })
        }
    };
    let book = db::Book::new(title, year).await?;
    println!("Inserted:");
    println!("{:?}", book);
    let link = db::AuthorBook::new(author.uuid(), book.uuid()).await?;
    println!("Linked:");
    println!("{:?}", link);
    Ok(())
}

pub async fn author_insert(name: &str, lang: &str) -> Result<()> {
    let book = db::Author::new(name, lang).await?;
    println!("{:?}", book);
    Ok(())
}

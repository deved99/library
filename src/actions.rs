use crate::db;
use crate::{Error, Result};

use std::ops::Deref;

// Book related 

pub async fn book_list() -> Result<()> {
    let books = db::BookComplete::list().await?;
    db::print_table(&books);
    Ok(())
}

pub async fn book_insert<T: Deref<Target = str>>(
    title: &str,
    author: &str,
    year: i16,
    tags: &[T],
) -> Result<()> {
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
    // Create links
    // // Author
    let author_link = db::AuthorBook::new(author.uuid(), book.uuid()).await?;
    // // Tag
    println!("Linked artist:\n {:?}", author_link);
    println!("Linking tags:");
    for tag in tags {
        db::Tag::find_or_create(tag).await?;
        let link = db::TagBook::new(tag, book.uuid()).await?;
        println!("{:?}", &link);
    }
    Ok(())
}

// Author related //

pub async fn author_list() -> Result<()> {
    let authors = db::AuthorComplete::list().await?;
    db::print_table(&authors);
    Ok(())
}

pub async fn author_insert(name: &str, lang: &str) -> Result<()> {
    let book = db::Author::new(name, lang).await?;
    println!("{:?}", book);
    Ok(())
}


// Tag related //
pub async fn tag_list() -> Result<()> {
    let tags = db::TagComplete::list().await?;
    db::print_table(&tags);
    Ok(())
}

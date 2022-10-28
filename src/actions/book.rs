use crate::db;
use crate::Result;
use chrono::{self, NaiveDate};
use std::ops::Deref;
use uuid::Uuid;

pub async fn list() -> Result<()> {
    let books = db::Book::list().await?;
    db::print_table(&books)?;
    Ok(())
}

pub async fn insert<T: Deref<Target = str>>(
    title: &str,
    author: &db::Author,
    year: i16,
    tags: &[T],
) -> Result<()> {
    let book = db::Book::new(title, year).await?;
    println!("Inserted:");
    println!("{:?}", book);
    // Create links
    // // Author
    let author_link = db::AuthorBook::write_new(author.uuid(), book.uuid()).await?;
    // // Tag
    println!("Linked artist:\n{:?}", author_link);
    println!("Linking tags:");
    for tag in tags {
        db::Tag::find_or_create(tag).await?;
        let link = db::TagBook::write_new(tag, book.uuid()).await?;
        println!("{:?}", &link);
    }
    Ok(())
}

pub async fn delete(uuid: Uuid) -> Result<()> {
    let book = db::Book::from_uuid(uuid).await?;
    db::AuthorBook::delete_about_book(uuid).await?;
    db::TagBook::delete_about_book(uuid).await?;
    book.delete().await
}

pub async fn start(uuid: Uuid, date: Option<NaiveDate>) -> Result<()> {
    let date = match date {
        None => chrono::Local::now().date_naive(),
        Some(d) => d,
    };
    let mut book = db::Book::from_uuid(uuid).await?;
    book.set_date_started(Some(date)).await?;
    Ok(())
}

pub async fn finish(uuid: Uuid, date: Option<NaiveDate>) -> Result<()> {
    let date = match date {
        None => chrono::Local::now().date_naive(),
        Some(d) => d,
    };
    let mut book = db::Book::from_uuid(uuid).await?;
    book.set_date_finished(Some(date)).await?;
    Ok(())
}

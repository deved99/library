use crate::db;
use crate::db::Author;
use crate::Result;
use chrono::{self, NaiveDate};
use futures::future::{join_all, try_join_all};
use std::ops::Deref;
use uuid::Uuid;

// CREATE //

pub async fn insert<T: Deref<Target = str>>(
    title: &str,
    author: &db::Author,
    year: Option<i16>,
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

// READ //

pub async fn list() -> Result<()> {
    let books = db::Book::list().await?;
    db::print_table(&books)?;
    Ok(())
}

// UPDATE //

pub async fn update(
    uuid: Uuid,
    title: Option<impl Deref<Target = str>>,
    year: Option<i16>,
    date_started: Option<NaiveDate>,
    date_finished: Option<NaiveDate>,
    authors: &[impl Deref<Target = str>],
    tags: &[impl Deref<Target = str>],
) -> Result<()> {
    let mut book = db::Book::from_uuid(uuid).await?;
    if let Some(x) = title {
        book.set_title(&x).await?;
    }
    if let Some(x) = year {
        book.set_year(x).await?;
    }
    if let Some(_) = date_started {
        book.set_date_started(date_started).await?;
    }
    if let Some(_) = date_finished {
        book.set_date_finished(date_finished).await?;
    }
    if authors.len() > 0 {
        db::AuthorBook::delete_about_book(uuid).await?;
        let authors_uuid = try_join_all(authors.iter().map(|a| db::Author::exists(a))).await?;
        for (author_name, author) in authors.iter().zip(authors_uuid) {
            match author {
                Some(a) => db::AuthorBook::write_new(a.uuid(), uuid).await?,
                None => {
                    log::warn!("Creating author {}.", author_name.deref());
                    let author = db::Author::new(author_name).await?;
                    db::AuthorBook::write_new(author.uuid(), uuid).await?
                }
            };
        }
    }
    if tags.len() > 0 {
        db::TagBook::delete_about_book(uuid).await?;
        for tag in tags {
            db::Tag::find_or_create(tag).await?;
            db::TagBook::write_new(tag, uuid).await?;
        }
    }
    Ok(())
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

pub async fn reset_date(uuid: Uuid) -> Result<()> {
    let mut book = db::Book::from_uuid(uuid).await?;
    book.set_date_started(None).await?;
    book.set_date_finished(None).await?;
    Ok(())
}

// DELETE //

pub async fn delete(uuid: Uuid) -> Result<()> {
    let book = db::Book::from_uuid(uuid).await?;
    db::AuthorBook::delete_about_book(uuid).await?;
    db::TagBook::delete_about_book(uuid).await?;
    book.delete().await
}

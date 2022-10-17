use std::include_str;
use std::ops::Deref;
use super::get_pool;
use super::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AuthorBook {
    author: Uuid,
    book: Uuid,
}
impl AuthorBook {
    pub fn new(author: Uuid, book: Uuid) -> Self {
        Self { author, book }
    }
    pub async fn write_new(author: Uuid, book: Uuid) -> Result<Self> {
        let db = get_pool().await?;
        let link: Self = sqlx::query_as(
            "INSERT INTO authors_books (author,book)
             VALUES ($1, $2)
             RETURNING author,book",
        )
        .bind(author)
        .bind(book)
        .fetch_one(db)
        .await?;
        Ok(link)
    }
    pub async fn write_many(links: &[Self]) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let query = include_str!("SQL/author-book_write-many.sql");
        let json = serde_json::to_string(links)?;
        let links: Vec<Self> = sqlx::query_as(query)
            .bind(json)
            .fetch_all(db)
            .await?;
        Ok(links)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct TagBook {
    tag: String,
    book: Uuid,
}
impl TagBook {
    pub fn new(tag: &str, book: Uuid) -> Self {
        let tag = tag.to_string();
        Self { tag, book}
    }
    pub async fn write_new(tag: &str, book: Uuid) -> Result<Self> {
        let db = get_pool().await?;
        let link: Self = sqlx::query_as(
            "INSERT INTO tags_books (tag,book)
             VALUES ($1, $2)
             RETURNING tag,book",
        )
        .bind(tag)
        .bind(book)
        .fetch_one(db)
        .await?;
        Ok(link)
    }
    pub async fn write_many(links: &[Self]) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let query = include_str!("SQL/tag-book_write-many.sql");
        let json = serde_json::to_string(links)?;
        let links: Vec<Self> = sqlx::query_as(query)
            .bind(json)
            .fetch_all(db)
            .await?;
        Ok(links)
    }
}

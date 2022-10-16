use super::get_pool;
use super::Result;
use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AuthorBook {
    author: Uuid,
    book: Uuid,
}
impl AuthorBook {
    pub async fn new(author: Uuid, book: Uuid) -> Result<Self> {
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
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct TagBook {
    tag: String,
    book: Uuid,
}
impl TagBook {
    pub async fn new(tag: &str, book: Uuid) -> Result<Self> {
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
}

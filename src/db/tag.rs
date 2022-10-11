use super::{get_pool, Result};
use sqlx;

#[derive(sqlx::FromRow)]
pub struct Tag {
    name: String,
}
impl Tag {
    pub async fn new(name: &str) -> Result<Self> {
        let db = get_pool().await?;
        let tag = sqlx::query_as("INSERT INTO tags (name) VALUES ($1) RETURNING name;")
            .bind(name)
            .fetch_one(db)
            .await?;
        Ok(tag)
    }
    pub async fn find(name: &str) -> Result<Option<Self>> {
        let db = get_pool().await?;
        let tag = sqlx::query_as("SELECT name FROM tags WHERE name = $1;")
            .bind(name)
            .fetch_optional(db)
            .await?;
        Ok(tag)
    }
    pub async fn find_or_create(name: &str) -> Result<Self> {
        let tag_maybe = Tag::find(name).await?;
        match tag_maybe {
            None => Tag::new(name).await,
            Some(t) => Ok(t),
        }
    }
}

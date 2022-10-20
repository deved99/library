use super::{get_pool, AsRow, Result};
use sqlx;
use serde::{Serialize, Deserialize};
use std::include_str;

#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Tag {
    name: String,
}
impl Tag {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let tags = sqlx::query_as("SELECT name FROM tags")
            .fetch_all(db)
            .await?;
        Ok(tags)
    }
    pub async fn write_new(name: &str) -> Result<Self> {
        let db = get_pool().await?;
        let tag = sqlx::query_as("INSERT INTO tags (name) VALUES ($1) RETURNING name;")
            .bind(name)
            .fetch_one(db)
            .await?;
        Ok(tag)
    }
    pub async fn write_many(tags: &[Self]) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let query = include_str!("SQL/tag_write-many.sql");
        let json = serde_json::to_string(tags)?;
        let tags: Vec<Self> = sqlx::query_as(query)
            .bind(json)
            .fetch_all(db)
            .await?;
        Ok(tags)
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
            None => Tag::write_new(name).await,
            Some(t) => Ok(t),
        }
    }
}

impl AsRow for Tag {
    fn titles() -> Vec<String> {
        vec![ String::from("name") ]
    }
    fn columns(&self) -> Vec<String> {
        vec![
            format!("{}", self.name),
        ]
    }
}

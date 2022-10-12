use std::include_str;
use super::{get_pool, AsRow, Result};
use sqlx;

#[derive(sqlx::FromRow, Debug)]
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


#[derive(sqlx::FromRow, Debug)]
pub struct TagComplete {
    tag: String,
    books: Vec<String>
}
impl TagComplete {
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let query = include_str!("SQL/tag-complete_list.sql");
        let results = sqlx::query_as(query).fetch_all(db).await?;
        Ok(results)
    }
}

impl AsRow for TagComplete {
    fn titles() -> Vec<String> {
        ["tag", "books"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
    fn columns(&self) -> Vec<String> {
        vec![
            format!("{}", self.tag),
            format!("{:?}", &self.books),
        ]
    }
}

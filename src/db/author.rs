use std::include_str;
use super::print_table;
use super::{get_pool, AsRow, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Author {
    uuid: Uuid,
    name: String,
    nationality: String,
}
impl Author {
    /// Create a new Author, writing it to the database.
    pub async fn new(name: &str, nationality: &str) -> Result<Self> {
        let db = get_pool().await?;
        let result: Self = sqlx::query_as(
            "INSERT INTO authors (name, nationality) VALUES ($1, $2)
             RETURNING uuid,name,nationality",
        )
        .bind(name)
        .bind(nationality)
        .fetch_one(db)
        .await?;
        Ok(result)
    }
    /// Write many authors to the database, returns written ones.
    pub async fn write_many(authors: &[Self]) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let query = include_str!("SQL/author_write-many.sql");
        let json = serde_json::to_string(authors)?;
        let authors: Vec<Self> = sqlx::query_as(query)
            .bind(json)
            .fetch_all(db)
            .await?;
        Ok(authors)
    }
    // Read
    pub async fn find(name: &str) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let result: Vec<Self> =
            sqlx::query_as("SELECT uuid,name,nationality FROM authors WHERE name = $1")
                .bind(name)
                .fetch_all(db)
                .await?;
        Ok(result)
    }
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let results: Vec<Self> = sqlx::query_as("SELECT * FROM authors;")
            .fetch_all(db)
            .await?;
        Ok(results)
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    // Update
    // // Helpers
    async fn update(&self) -> Result<()> {
        let db = get_pool().await?;
        sqlx::query("UPDATE authors SET name = $2, nationality = $3, WHERE uuid = $1")
            .bind(self.uuid)
            .bind(&self.name)
            .bind(&self.nationality)
            .execute(db)
            .await?;
        Ok(())
    }
}

impl AsRow for Author {
    fn titles() -> Vec<String> {
        ["uuid", "author", "nationality"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
    fn columns(&self) -> Vec<String> {
        vec![
            format!("{}", self.uuid),
            format!("{}", self.name),
            format!("{}", self.nationality),
        ]
    }
}


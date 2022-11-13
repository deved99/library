use super::{get_pool, AsRow, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx;
use std::include_str;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Author {
    uuid: Uuid,
    display_name: String,
    order_name: String,
}
impl Author {
    /// Create a new Author, writing it to the database.
    pub async fn new(display_name: &str, order_name: Option<&str>) -> Result<Self> {
        let db = get_pool().await?;
        let result = sqlx::query_as!(
            Self,
            "INSERT INTO authors (display_name,order_name) VALUES ($1, $2)
             RETURNING uuid, display_name, order_name",
            display_name,
            order_name.unwrap_or(display_name)
        )
        .fetch_one(db)
        .await?;
        Ok(result)
    }
    /// Write many authors to the database, returns written ones.
    pub async fn write_many(authors: &[Self]) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let json = serde_json::to_value(authors)?;
        let authors = sqlx::query_file_as!(Self, "SQL/author_write-many.sql", json)
            .fetch_all(db)
            .await?;
        Ok(authors)
    }
    // Read
    pub async fn exists(name: &str) -> Result<Option<Self>> {
        let db = get_pool().await?;
        let result = sqlx::query_as!(
            Self,
            "SELECT uuid, display_name, order_name FROM authors WHERE display_name = $1",
            name
        )
        .fetch_optional(db)
        .await?;
        Ok(result)
    }
    pub async fn from_uuid(uuid: Uuid) -> Result<Self> {
        let db = get_pool().await?;
        let result = sqlx::query_as!(
            Self,
            "SELECT uuid, display_name, order_name FROM authors WHERE uuid = $1",
            uuid
        )
        .fetch_one(db)
        .await?;
        Ok(result)
    }
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let results = sqlx::query_as!(Self, "SELECT uuid, display_name, order_name FROM authors")
            .fetch_all(db)
            .await?;
        Ok(results)
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    // Update
    pub fn update_display_name(&mut self, display_name: &str) {
        self.display_name = display_name.to_string();
    }
    pub fn update_order_name(&mut self, order_name: &str) {
        self.order_name = order_name.to_string();
    }
    // // Helpers
    pub async fn save(&self) -> Result<()> {
        let db = get_pool().await?;
        sqlx::query!(
            "UPDATE authors SET display_name = $2, order_name = $3 WHERE uuid = $1",
            self.uuid,
            &self.display_name,
            &self.order_name,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    // DELETE //

    pub async fn delete(&self) -> Result<()> {
        let db = get_pool().await?;
        sqlx::query!(
            "DELETE FROM authors
             WHERE uuid = $1",
            self.uuid
        )
        .execute(db)
        .await?;
        Ok(())
    }
}

impl AsRow for Author {
    fn titles() -> Vec<String> {
        ["uuid", "display_name", "order_name"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
    fn columns(&self) -> Vec<String> {
        vec![
            format!("{}", self.uuid),
            format!("{}", self.display_name),
            format!("{}", self.order_name),
        ]
    }
}

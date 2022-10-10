use super::{get_pool, Result};
use sqlx;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Author {
    uuid: Uuid,
    name: String,
    nationality: String,
}
impl Author {
    // Create
    pub async fn new(name: &str, nationality: &str) -> Result<Self> {
        let db = get_pool().await?;
        let result: Self = sqlx::query_as(
            "INSERT INTO authors (name, nationality)
             VALUES ($1, $2)
             RETURNING uuid,name,nationality",
        )
        .bind(name)
        .bind(nationality)
        .fetch_one(db)
        .await?;
        Ok(result)
    }
    pub async fn find(name: &str) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let result: Vec<Self> = sqlx::query_as(
            "SELECT uuid,name,nationality
             FROM authors
             WHERE name = $1",
        )
        .bind(name)
        .fetch_all(db)
        .await?;
        Ok(result)
    }
    // Read
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

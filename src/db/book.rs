use super::{get_pool, Result};
use sqlx;
use uuid::Uuid;

#[derive(Debug,sqlx::FromRow)]
pub struct Book {
    uuid: Uuid,
    title: String,
    year: i16,
    state: ReadingState,
}
impl Book {
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let results: Vec<Self> = sqlx::query_as("SELECT * FROM books;")
            .fetch_all(db)
            .await?;
        Ok(results)
    }
}

#[derive(Debug,sqlx::Type)]
#[sqlx(type_name = "reading_state", rename_all = "snake_case")]
enum ReadingState {
    Finished,
    ToRead,
    Reading
}

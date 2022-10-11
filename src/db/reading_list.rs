use super::{book, get_pool, Result, AsRow};
use sqlx;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct ReadingList {
    title: String,
    author: String,
    state: book::ReadingState,
}
impl ReadingList {
    pub async fn get() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let reading_list = sqlx::query_as(
            "SELECT a.name as author, b.title, b.state
             FROM books as b
             LEFT JOIN authors_books as l ON l.book = b.uuid
             LEFT JOIN authors as a ON a.uuid = l.author;",
        )
        .fetch_all(db)
        .await?;
        Ok(reading_list)
    }
}
impl AsRow for ReadingList {
    fn titles() -> Vec<String> {
        ["title", "author", "state"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
    fn columns(&self) -> Vec<String> {
        vec![
            format!("{}", self.author),
            format!("{}", self.title),
            format!("{:?}", self.state),
        ]
    }
}

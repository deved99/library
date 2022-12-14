use crate::db::{self, get_pool, AsRow, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Book {
    uuid: Uuid,
    title: String,
    date_started: Option<NaiveDate>,
    date_finished: Option<NaiveDate>,
    year: Option<i16>,
}

impl Book {
    // CREATE //
    pub async fn new(title: &str, year: Option<i16>) -> Result<Self> {
        let db = get_pool().await?;
        let result: Self = sqlx::query_as!(
            Self,
            "INSERT INTO books (title, year)
             VALUES ($1, $2)
             RETURNING uuid, title, year, date_started, date_finished",
            title,
            year
        )
        .fetch_one(db)
        .await?;
        Ok(result)
    }

    // READ //

    pub async fn from_uuid(uuid: Uuid) -> Result<Self> {
        let db = get_pool().await?;
        let book = sqlx::query_as!(
            Self,
            "SELECT uuid, title, year, date_started, date_finished
             FROM books
             WHERE uuid = $1",
            uuid
        )
        .fetch_one(db)
        .await?;
        Ok(book)
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let results: Vec<Self> = sqlx::query_as!(
            Self,
            "SELECT uuid, title, year, date_started, date_finished FROM books"
        )
        .fetch_all(db)
        .await?;
        Ok(results)
    }
    pub async fn write_many(books: &[Self]) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let json = serde_json::to_value(books)?;
        let books: Vec<Self> = sqlx::query_file_as!(Self, "SQL/book_write-many.sql", json)
            .fetch_all(db)
            .await?;
        Ok(books)
    }

    // UPDATE //

    pub async fn set_title(&mut self, title: &str) -> Result<()> {
        self.title = title.to_string();
        self.update().await
    }

    pub async fn set_year(&mut self, year: i16) -> Result<()> {
        self.year = Some(year);
        self.update().await
    }

    /// Set self.started_reading
    pub async fn set_date_started(&mut self, date: Option<NaiveDate>) -> Result<()> {
        self.date_started = date;
        self.update().await
    }
    /// Set self.finished_reading
    pub async fn set_date_finished(&mut self, date: Option<NaiveDate>) -> Result<()> {
        self.date_finished = date;
        self.update().await
    }

    async fn update(&self) -> Result<()> {
        let db = get_pool().await?;
        sqlx::query!(
            "UPDATE books
             SET title = $2, year = $3, date_started = $4, date_finished = $5
             WHERE uuid = $1",
            self.uuid,
            &self.title,
            self.year,
            self.date_started,
            self.date_finished
        )
        .execute(db)
        .await?;
        Ok(())
    }

    // DELETE //

    pub async fn delete(&self) -> Result<()> {
        let db = get_pool().await?;
        sqlx::query!(
            "DELETE FROM books
             WHERE uuid = $1",
            self.uuid
        )
        .execute(db)
        .await?;
        Ok(())
    }
}

impl AsRow for Book {
    fn titles() -> Vec<String> {
        ["uuid", "title", "year", "date_started", "date_finished"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
    fn columns(&self) -> Vec<String> {
        let year_str = match self.year {
            Some(n) => n.to_string(),
            None => String::new(),
        };
        let display_optional_date = |date: Option<NaiveDate>| match date {
            None => "".to_string(),
            Some(d) => d.to_string(),
        };
        vec![
            format!("{}", self.uuid),
            format!("{}", self.title),
            year_str,
            display_optional_date(self.date_started),
            display_optional_date(self.date_finished),
        ]
    }
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct BookDump {
    uuid: Uuid,
    title: String,
    year: Option<i16>,
    date_started: Option<NaiveDate>,
    date_finished: Option<NaiveDate>,
    authors: Vec<Uuid>,
    tags: Vec<String>,
}

impl BookDump {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let books: Vec<Self> = sqlx::query_file_as_unchecked!(Self, "SQL/book-dump_list.sql")
            .fetch_all(db)
            .await?;
        return Ok(books);
    }
    pub fn to_book(&self) -> Book {
        Book {
            uuid: self.uuid,
            title: self.title.to_string(),
            year: self.year,
            date_started: self.date_started,
            date_finished: self.date_finished,
        }
    }
    pub fn to_tags(&self) -> Vec<db::Tag> {
        self.tags.iter().map(|x| db::Tag::new(x)).collect()
    }
    pub fn to_tag_links(&self) -> Vec<db::TagBook> {
        self.tags
            .iter()
            .map(|x| db::TagBook::new(x, self.uuid))
            .collect()
    }
    pub fn to_author_links(&self) -> Vec<db::AuthorBook> {
        self.authors
            .iter()
            .map(|x| db::AuthorBook::new(*x, self.uuid))
            .collect()
    }
}

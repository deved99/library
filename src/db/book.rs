use crate::db::{self, get_pool, AsRow, Result};
use serde::{Deserialize, Serialize};
use sqlx;
use time::Date;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Book {
    uuid: Uuid,
    title: String,
    year: i16,
    date_started: Option<Date>,
    date_finished: Option<Date>,
}
impl Book {
    // Create
    pub async fn new(title: &str, year: i16) -> Result<Self> {
        let db = get_pool().await?;
        let result: Self = sqlx::query_as(
            "INSERT INTO books (title, year) VALUES ($1, $2) RETURNING uuid,title,year,date_started,date_finished",
        )
        .bind(title)
        .bind(year)
        .fetch_one(db)
        .await?;
        Ok(result)
    }
    // Read
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let results: Vec<Self> = sqlx::query_as("SELECT * FROM books;").fetch_all(db).await?;
        Ok(results)
    }
    /// Write many books to the database, returns written ones.
    pub async fn write_many(books: &[Self]) -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let query = include_str!("SQL/book_write-many.sql");
        let json = serde_json::to_string(books)?;
        let books: Vec<Self> = sqlx::query_as(query)
            .bind(json)
            .fetch_all(db)
            .await?;
        Ok(books)
    }
    // Update
    //// state change
    // pub async fn read(&mut self) -> Result<()> {
    //     self.state = ReadingState::Reading;
    //     self.update().await
    // }
    // pub async fn finish(&mut self) -> Result<()> {
    //     self.state = ReadingState::Finished;
    //     self.update().await
    // }
    // pub async fn reset(&mut self) -> Result<()> {
    //     self.state = ReadingState::ToRead;
    //     self.update().await
    // }
    //// Helpers
    async fn update(&self) -> Result<()> {
        let db = get_pool().await?;
        sqlx::query("UPDATE books SET title = $2, year = $3, date_started = $4, date_finished = $5 WHERE uuid = $1")
            .bind(self.uuid)
            .bind(&self.title)
            .bind(self.year)
            .bind(self.date_started)
            .bind(self.date_finished)
            .execute(db)
            .await?;
        Ok(())
    }
}

impl AsRow for Book {
    fn titles() -> Vec<String> {
        ["title", "year", "date_started", "date_finished"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
    fn columns(&self) -> Vec<String> {
        vec![
            format!("{}", self.title),
            format!("{}", self.year),
            format!("{:?}", self.date_started),
            format!("{:?}", self.date_finished),
        ]
    }
}

#[derive(Debug, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "reading_state", rename_all = "snake_case")]
pub enum ReadingState {
    Finished,
    ToRead,
    Reading,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct BookComplete {
    uuid: Uuid,
    title: String,
    year: i16,
    date_started: Option<Date>,
    date_finished: Option<Date>,
    authors: Vec<String>,
    tags: Vec<String>,
}

impl BookComplete {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let query = include_str!("SQL/book-complete_list.sql");
        let books = sqlx::query_as(query).fetch_all(db).await?;
        return Ok(books);
    }
}

impl AsRow for BookComplete {
    fn titles() -> Vec<String> {
        ["title", "author", "year", "tags"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
    fn columns(&self) -> Vec<String> {
        vec![
            format!("{}", self.title),
            format!("{:?}", self.authors),
            format!("{}", self.year),
            format!("{:?}", &self.tags),
        ]
    }
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct BookDump {
    uuid: Uuid,
    title: String,
    year: i16,
    date_started: Option<Date>,
    date_finished: Option<Date>,
    authors: Vec<Uuid>,
    tags: Vec<String>,
}

impl BookDump {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    pub async fn list() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let query = include_str!("SQL/book-dump_list.sql");
        let books = sqlx::query_as(query).fetch_all(db).await?;
        return Ok(books);
    }
    pub fn to_book(&self) -> Book {
        Book {
            uuid: self.uuid,
            title: self.title.to_string(),
            year: self.year,
            date_started: self.date_started,
            date_finished: self.date_finished
        }
    }
    pub fn to_tags(&self) -> Vec<db::Tag> {
        self.tags.iter()
            .map(|x| db::Tag::new(x))
            .collect()
    }
    pub fn to_tag_links(&self) -> Vec<db::TagBook> {
        self.tags.iter()
            .map(|x| db::TagBook::new(x, self.uuid))
            .collect()
    }
    pub fn to_author_links(&self) -> Vec<db::AuthorBook> {
        self.authors.iter()
            .map(|x| db::AuthorBook::new(*x, self.uuid))
            .collect()
    }
}

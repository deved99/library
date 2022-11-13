use crate::miscutils::{add_default, ask, confirm};
use crate::{actions, db, Result};
use chrono;
use clap::{Args, Subcommand};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum Book {
    /// List books in the reading list
    List,
    /// Insert a book in the database
    Insert(InsertBook),
    /// Delete a book from the database
    Delete { uuid: Uuid },
    /// Update a book
    Update(UpdateBook),
    /// Start a book
    Start {
        uuid: Uuid,
        #[arg(long)]
        date: Option<chrono::NaiveDate>,
    },
    /// Finish a book
    Finish {
        uuid: Uuid,
        #[arg(long)]
        date: Option<chrono::NaiveDate>,
    },
    /// Reset started_date and finished date of a book
    #[command(name = "date-reset")]
    DateReset { uuid: Uuid },
}

impl Book {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book::list().await,
            Self::Insert(b) => b.execute().await,
            Self::Delete { uuid } => actions::book::delete(uuid).await,
            Self::Update(update_book) => update_book.execute().await,
            Self::Start { uuid, date } => actions::book::start(uuid, date).await,
            Self::Finish { uuid, date } => actions::book::finish(uuid, date).await,
            Self::DateReset { uuid } => actions::book::reset_date(uuid).await,
        }
    }
}

#[derive(Args, Debug)]
pub struct InsertBook {
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    author: Option<String>,
    #[arg(long)]
    year: Option<i16>,
    #[arg(long)]
    tag: Vec<String>,
}

impl InsertBook {
    async fn execute(mut self) -> Result<()> {
        log::debug!("Inserting book: initial values: {:?}", &self);
        self.fill()?;
        // Now I can unwrap
        let title = self.title.unwrap();
        let author = self.author.unwrap();
        // Ensure the choosen author exists, else create it
        let author = match db::Author::exists(&author).await? {
            Some(a) => a,
            None => db::Author::new(&author).await?,
        };
        // Finally insert
        actions::book::insert(&title, &author, self.year, &self.tag).await
    }
    fn fill(&mut self) -> Result<()> {
        while self.is_something_missing() || !self.confirm() {
            // Set title
            let title = ask(&add_default("Title?", &self.title))?;
            if !title.is_empty() {
                self.title = Some(title);
            }
            // Set author
            let author = ask(&add_default("Author?", &self.author))?;
            if !author.is_empty() {
                self.author = Some(author);
            }
            println!("");
        }
        Ok(())
    }
    fn confirm(&self) -> bool {
        // println!("Tha preview");
        match confirm(true) {
            Ok(b) => b,
            Err(e) => {
                log::error!("{}", e);
                false
            }
        }
    }
    fn is_something_missing(&self) -> bool {
        self.title.is_none() || self.author.is_none()
    }
}

#[derive(Args, Debug)]
pub struct UpdateBook {
    uuid: Uuid,
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    year: Option<i16>,
    #[arg(long)]
    date_started: Option<chrono::NaiveDate>,
    #[arg(long)]
    date_finished: Option<chrono::NaiveDate>,
    #[arg(long)]
    author: Vec<String>,
    #[arg(long)]
    tag: Vec<String>,
}
impl UpdateBook {
    async fn execute(self) -> Result<()> {
        actions::book::update(
            self.uuid,
            self.title,
            self.year,
            self.date_started,
            self.date_finished,
            &self.author,
            &self.tag,
        )
        .await
    }
}

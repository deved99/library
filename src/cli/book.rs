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
    Delete {
        uuid: Uuid,
    },
    /// Start a book
    Start {
        #[arg(long)]
        uuid: Uuid,
        #[arg(long)]
        date: Option<chrono::NaiveDate>,
    },
    /// Finish a book
    Finish {
        #[arg(long)]
        uuid: Uuid,
        #[arg(long)]
        date: Option<chrono::NaiveDate>,
    },
}

impl Book {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::List => actions::book::list().await,
            Self::Insert(b) => b.execute().await,
            Self::Delete { uuid } => actions::book::delete(uuid).await,
            Self::Start { uuid, date } => actions::book::start(uuid, date).await,
            Self::Finish { uuid, date } => actions::book::finish(uuid, date).await,
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
        let year = self.year.unwrap();
        let author = self.author.unwrap();
        // Ensure the choosen author exists, else create it
        let author = match db::Author::exists(&author).await? {
            Some(a) => a,
            None => {
                let mut nationality = String::new();
                while nationality.is_empty() || !confirm(true)? {
                    nationality = ask("Nationality?")?;
                }
                db::Author::new(&author, &nationality).await?
            }
        };
        // Finally insert
        actions::book::insert(&title, &author, year, &self.tag).await
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
            // Set year
            let year = ask(&add_default("Year?", &self.year))?;
            match year.parse::<i16>() {
                Err(e) => {
                    log::error!("Error: {}", e);
                }
                Ok(n) => {
                    self.year = Some(n);
                }
            };
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
        self.title.is_none() || self.year.is_none() || self.author.is_none()
    }
}

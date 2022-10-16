use std::collections::HashSet;

use super::{Author, BookComplete};
use crate::Result;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Dump {
    books: Vec<BookComplete>,
    authors: Vec<Author>,
}

impl Dump {
    pub async fn get() -> Result<Self> {
        let books = BookComplete::list().await?;
        let authors = Author::list().await?;
        Ok(Self { books, authors })
    }
    pub async fn write(&self) -> Result<()> {
        // init:
        // - tags: stores which tag should be created
        // - authors_books: stores the links author,book
        // - tags_books: stores the links tag,book
        // let mut tags = HashSet::new();
        // let mut authors_books = HashSet::new();
        // let mut tags_books = HashSet::new();

        // Write authors
        Author::write_many(&self.authors).await?;

        // iterate on book, fill tags, authors_books, tags_books

        // Create tags and links

        Ok(())
    }
}

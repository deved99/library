use std::collections::HashSet;

use super::{Author, Book, BookDump};
use crate::Result;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Dump {
    books: Vec<BookDump>,
    authors: Vec<Author>,
}

impl Dump {
    pub async fn export() -> Result<Self> {
        let books = BookDump::list().await?;
        let authors = Author::list().await?;
        Ok(Self { books, authors })
    }
    pub async fn import(&self) -> Result<()> {
        // Process &self to:
        let authors_books: HashSet<(Uuid, Uuid)> = self.books.iter()
            .map(|x| x.to_author_links())
            .flatten()
            .collect();
        let tags_books: HashSet<(Uuid, String)> = self.books.iter()
            .map(|x| x.to_tag_links())
            .flatten()
            .collect();
        let tags: HashSet<&String> = tags_books.iter()
            .map(|x| &x.1)
            .collect();
        let books: Vec<Book> = self.books.iter()
            .map(|x| x.to_book())
            .collect();

        // Write everything
        Author::write_many(&self.authors).await?;
        Book::write_many(&books).await?;

        Ok(())
    }
}

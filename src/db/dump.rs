use std::collections::HashSet;

use crate::db::{self, Author, Book, BookDump, Tag, AuthorBook, TagBook};
use crate::Result;

use itertools::Itertools;
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
        let books: Vec<Book> = self.books.iter()
            .map(|x| x.to_book())
            .unique()
            .collect();
        let tags: Vec<db::Tag> = self.books.iter()
            .map(|x| x.to_tags())
            .flatten()
            .unique()
            .collect();
        let authors_books: Vec<AuthorBook> = self.books.iter()
            .map(|x| x.to_author_links())
            .flatten()
            .unique()
            .collect();
        let tags_books: Vec<TagBook> = self.books.iter()
            .map(|x| x.to_tag_links())
            .flatten()
            .unique()
            .collect();

        // Write everything
        Author::write_many(&self.authors).await?;
        Book::write_many(&books).await?;
        Tag::write_many(&tags).await?;
        AuthorBook::write_many(&authors_books).await?;
        TagBook::write_many(&tags_books).await?;

        Ok(())
    }
}

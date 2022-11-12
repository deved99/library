-- Cleanup
DROP VIEW IF EXISTS reading_list;
DROP TABLE IF EXISTS authors_books;
DROP TABLE IF EXISTS tags_books;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS authors;
DROP TABLE IF EXISTS tags;

-- Add uuid functions

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Actual tables

CREATE TABLE books(
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  CONSTRAINT uuid_books PRIMARY KEY ( uuid ),
  title TEXT NOT NULL,
  -- state reading_state NOT NULL DEFAULT 'to_read'
  date_started DATE,
  date_finished DATE,
  -- Additional data
  year SMALLINT
);

CREATE TABLE authors(
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  CONSTRAINT uuid_authors PRIMARY KEY ( uuid ),
  display_name TEXT NOT NULL,
  order_name TEXT NOT NULL
);

CREATE TABLE authors_books(
  author UUID NOT NULL REFERENCES authors (uuid),
  book UUID NOT NULL REFERENCES books (uuid)
);

CREATE TABLE tags(
  name TEXT NOT NULL,
  CONSTRAINT name_tags PRIMARY KEY ( name )
);

CREATE TABLE tags_books(
  tag TEXT NOT NULL REFERENCES tags (name),
  book UUID NOT NULL REFERENCES books (uuid)
);

-- Views

CREATE VIEW reading_list AS
SELECT b.uuid,
    b.title,
    b.date_started,
    b.date_finished,
    COALESCE(la.display_authors, '{}'::text[]) AS display_authors,
    COALESCE(la.order_authors, '{}'::text[]) AS order_authors,
    COALESCE(lt.tags, '{}'::text[]) AS tags
FROM books AS b
LEFT JOIN (
        SELECT
            l.book,
            array_agg(a.display_name) AS display_authors,
            array_agg(a.order_name) AS order_authors
        FROM authors_books l
        LEFT JOIN authors a ON a.uuid = l.author
        GROUP BY l.book
    ) AS la ON b.uuid = la.book
LEFT JOIN (
        SELECT tags_books.book, array_agg(tags_books.tag) AS tags
        FROM tags_books
        GROUP BY tags_books.book
    ) lt ON b.uuid = lt.book;

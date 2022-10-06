-- Cleanup
DROP TABLE IF EXISTS authors_books;
DROP TABLE IF EXISTS tags_books;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS authors;
DROP TABLE IF EXISTS tags;
DROP TYPE IF EXISTS reading_state;

-- Add uuid functions

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Actual tables

CREATE TYPE reading_state AS ENUM ('finished', 'to_read', 'reading');
CREATE TABLE books(
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  CONSTRAINT uuid_books PRIMARY KEY ( uuid ),
  title TEXT NOT NULL,
  year SMALLINT NOT NULL,
  state reading_state NOT NULL
);

CREATE TABLE authors(
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  CONSTRAINT uuid_authors PRIMARY KEY ( uuid ),
  name TEXT,
  nationality TEXT
);

CREATE TABLE authors_books(
  author UUID NOT NULL REFERENCES authors (uuid),
  book UUID NOT NULL REFERENCES books (uuid)
);

CREATE TABLE tags(
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  CONSTRAINT uuid_tags PRIMARY KEY ( uuid ),
  name TEXT
);

CREATE TABLE tags_books(
  tag UUID NOT NULL REFERENCES tags (uuid),
  book UUID NOT NULL REFERENCES books (uuid)
);

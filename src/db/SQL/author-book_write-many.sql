INSERT INTO authors_books(author, book)
SELECT author, book
FROM json_to_recordset($1::json) AS d("author" UUID, "book" UUID)
WHERE (author, book) NOT IN (SELECT author, book FROM authors_books)
RETURNING author, book

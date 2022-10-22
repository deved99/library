INSERT into tags_books(tag, book)
SELECT tag, book
FROM json_to_recordset($1::json) AS d("tag" TEXT, "book" UUID)
WHERE (tag, book) NOT IN (SELECT tag, book  FROM tags_books)
RETURNING tag, book

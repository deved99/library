INSERT into books (uuid, title, date_started, date_finished, year)
SELECT uuid, title, date_started, date_finished, year
FROM json_to_recordset($1::json) AS d("uuid" UUID, "title" TEXT, "year" INTEGER, "date_started" DATE, "date_finished" DATE)
WHERE uuid NOT IN (SELECT uuid FROM books)
RETURNING uuid, title, year, date_started, date_finished

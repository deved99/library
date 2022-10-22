INSERT into authors
SELECT uuid, name, nationality
FROM json_to_recordset($1::json) AS d("uuid" UUID, "name" TEXT, "nationality" TEXT)
WHERE uuid NOT IN (SELECT uuid FROM authors)
RETURNING uuid, name, nationality

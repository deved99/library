INSERT into authors
SELECT uuid, name
FROM json_to_recordset($1::json) AS d("uuid" UUID, "name" TEXT)
WHERE uuid NOT IN (SELECT uuid FROM authors)
RETURNING uuid, name

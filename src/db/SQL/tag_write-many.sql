INSERT into tags
SELECT name
FROM json_to_recordset($1::json) AS d("name" TEXT)
WHERE name NOT IN (SELECT name  FROM tags)
RETURNING name

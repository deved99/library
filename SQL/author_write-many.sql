INSERT into authors
SELECT uuid, display_name, order_name
FROM json_to_recordset($1::json) AS d("uuid" UUID, "display_name" TEXT, "order_name" TEXT)
WHERE uuid NOT IN (SELECT uuid FROM authors)
RETURNING uuid, display_name, order_name

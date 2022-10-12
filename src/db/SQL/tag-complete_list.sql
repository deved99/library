SELECT t.name AS tag, coalesce(l.books, '{}') AS books
FROM tags AS t

LEFT JOIN (
    SELECT l_raw.tag, array_agg(b.title) AS books
    FROM tags_books AS l_raw
    LEFT JOIN books as b ON l_raw.book = b.uuid
    GROUP BY l_raw.tag
) AS l ON t.name = l.tag

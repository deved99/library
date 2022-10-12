SELECT
    a.name, a.nationality,
    coalesce(l.books, '{}') AS books
FROM authors AS a

LEFT JOIN (
    SELECT author, array_agg(b.title) AS books
    FROM authors_books as l
    LEFT JOIN books as b ON b.uuid = l.book
    GROUP BY author
) AS l ON a.uuid = l.author

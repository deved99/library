-- uuid, title, author, tags, year
SELECT
    b.uuid, b.title, b.year, b.date_started, b.date_finished,
    coalesce(la.authors, '{}') AS authors,
    coalesce(lt.tags, '{}') AS tags
FROM books as b

LEFT JOIN (
    SELECT book, array_agg(a.uuid) AS authors
    FROM authors_books as l
    LEFT JOIN authors as a ON a.uuid = l.author
    GROUP BY book
) AS la ON b.uuid = la.book

LEFT JOIN (
    SELECT book, array_agg(tag) AS tags FROM tags_books
    GROUP BY book
) AS lt ON b.uuid = lt.book

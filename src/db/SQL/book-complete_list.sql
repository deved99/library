-- uuid, title, author, tags, year
SELECT b.uuid, b.title, b.year, array_agg(a.name) as authors, array_agg(lt.tag) as tags
FROM books as b
LEFT JOIN authors_books as la ON b.uuid = la.book
LEFT JOIN authors as a ON a.uuid = la.author
LEFT JOIN tags_books as lt ON b.uuid = lt.book
GROUP BY b.uuid;

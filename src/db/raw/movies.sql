SELECT id,
       title,
       description,
       ts_rank_cd(_document, query) AS rank
FROM movies,
     plainto_tsquery(CAST('english' AS REGCONFIG), CAST('{}' AS TEXT)) query
WHERE _document @@ query AND {}
ORDER BY rank DESC
LIMIT {}
OFFSET {};
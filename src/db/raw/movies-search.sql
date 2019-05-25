SELECT movie_id,
       title,
       description,
       poster,
       ts_rank_cd(_document, query) AS rank
FROM movies,
     websearch_to_tsquery(CAST('english' AS REGCONFIG), CAST('{}' AS TEXT)) query
WHERE _document @@ query {} -- Movie filters
ORDER BY rank DESC
LIMIT {}
OFFSET {};
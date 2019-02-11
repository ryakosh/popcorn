SELECT id, title, description, ts_rank_cd(document, ?) AS rank
FROM movies
WHERE document @@ websearch_to_tsquery('english', ?)
ORDER BY rank DESC
LIMIT ?
OFFSET ?;
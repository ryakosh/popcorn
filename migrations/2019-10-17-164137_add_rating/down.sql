DROP TABLE IF EXISTS users_ratings;

ALTER TABLE movies
    DROP COLUMN rating,
    DROP COLUMN rating_count;
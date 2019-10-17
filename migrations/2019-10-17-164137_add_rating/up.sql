CREATE TABLE users_ratings (
    user_id INTEGER NOT NULL,
    movie_id INTEGER NOT NULL,
    user_rating SMALLINT NOT NULL,
    PRIMARY KEY(user_id, movie_id)
);

ALTER TABLE movies
    ADD COLUMN rating REAL NOT NULL DEFAULT 0.0,
    ADD COLUMN rating_count INTEGER NOT NULL DEFAULT 0;
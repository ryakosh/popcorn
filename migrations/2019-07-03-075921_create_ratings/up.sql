CREATE TABLE ratings (
user_id VARCHAR,
movie_id INTEGER,
rating INTEGER NOT NULL,
PRIMARY KEY(user_id, movie_id)
);
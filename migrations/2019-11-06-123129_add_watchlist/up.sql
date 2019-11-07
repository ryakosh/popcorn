CREATE TABLE users_watchlist(
    user_id VARCHAR REFERENCES users(id) ON DELETE CASCADE,
    movie_id INTEGER REFERENCES movies(movie_id) ON DELETE CASCADE,
    PRIMARY KEY(user_id, movie_id)
);
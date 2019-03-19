CREATE TABLE movies (
  movie_id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL DEFAULT 'Description is not provided.',
  poster VARCHAR(19),
  genres TEXT[],
  languages TEXT[],
  release_country VARCHAR(2),
  release_date DATE,
  duration SMALLINT
);

CREATE TABLE movies_writers (
  movie_id INTEGER,
  writer_id INTEGER,
  PRIMARY KEY(movie_id, writer_id)
);

CREATE TABLE movies_directors (
  movie_id INTEGER,
  director_id INTEGER,
  PRIMARY KEY(movie_id, director_id)
);

CREATE TABLE movies_artists (
  movie_id INTEGER,
  artist_id INTEGER,
  PRIMARY KEY(movie_id, artist_id)
);

CREATE INDEX idx_genres ON movies USING GIN(genres);
CREATE INDEX idx_languages ON movies USING GIN(languages);
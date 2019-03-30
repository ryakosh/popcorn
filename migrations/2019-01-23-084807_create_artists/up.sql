CREATE TABLE artists (
  artist_id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  gender VARCHAR(1) NOT NULL
);
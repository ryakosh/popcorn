CREATE TABLE movies (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR,
  genres TEXT[],
  languages TEXT[],
  release_country VARCHAR(2),
  release_date DATE,
  duration SMALLINT,
  directors INT[],
  writers INT[],
  stars INT[]
);
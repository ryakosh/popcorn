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

CREATE INDEX idx_genres ON movies USING GIN(genres);
CREATE INDEX idx_languages ON movies USING GIN(languages);
CREATE INDEX idx_directors ON movies USING GIN(directors);
CREATE INDEX idx_writers ON movies USING GIN(writers);
CREATE INDEX idx_stars ON movies USING GIN(stars);
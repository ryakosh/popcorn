use crate::chrono::NaiveDate;
use super::schema::{users, movies};

#[table_name = "users"]
#[derive(Queryable, Insertable, Serialize)]
pub struct User {
  pub id: String,
  pub email: String,
  pub pwd: String,
}

#[derive(Queryable, Serialize)]
pub struct Artist {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub gender: String,
}

#[derive(Queryable, Serialize)]
pub struct Director {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub gender: String,
}

#[derive(Queryable, Serialize)]
pub struct Writer {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub gender: String,
}

#[derive(Queryable)]
pub struct Movie {
  movie_id: i32,
  title: String,
  description: String,
  poster: String,
  genres: Vec<String>,
  languages: Vec<String>,
  release_country: String,
  #[serde(skip)]
  release_date: NaiveDate,
  duration: i16,
}

#[table_name = "movies"]
#[derive(QueryableByName, Serialize)]
pub struct MovieCompact {
  movie_id: i32,
  title: String,
  description: String,
}
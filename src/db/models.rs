use crate::chrono::NaiveDate;
use super::schema::{users, movies};

#[table_name = "users"]
#[derive(Queryable, Insertable, Serialize)]
pub struct User {
  pub id: String,
  pub email: String,
  pub pwd: String,
}

#[derive(Queryable)]
pub struct Artist<'a> {
  pub id: i32,
  pub first_name: &'a str,
  pub last_name: &'a str,
  pub gender: &'a str,
}

#[derive(Queryable)]
pub struct Director<'d> {
  pub id: i32,
  pub first_name: &'d str,
  pub last_name: &'d str,
  pub gender: &'d str,
}

#[derive(Queryable)]
pub struct Writer<'w> {
  pub id: i32,
  pub first_name: &'w str,
  pub last_name: &'w str,
  pub gender: &'w str,
}

#[derive(Queryable, Serialize)]
pub struct Movie {
  movie_id: i32,
  title: String,
  description: String,
  poster: Option<String>,
  genres: Option<Vec<String>>,
  languages: Option<Vec<String>>,
  release_country: Option<String>,
  #[serde(skip)]
  release_date: Option<NaiveDate>,
  duration: Option<i16>,
}

#[table_name = "movies"]
#[derive(QueryableByName, Serialize)]
pub struct MovieCompact {
  movie_id: i32,
  title: String,
  description: String,
}
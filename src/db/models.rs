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

#[derive(Queryable, Serialize)]
pub struct Movie {
  pub movie_id: i32,
  pub title: String,
  pub description: String,
  pub poster: String,
  pub genres: Vec<String>,
  pub languages: Vec<String>,
  pub release_country: String,
  pub release_date: NaiveDate,
  pub duration: i16,
}

#[table_name = "movies"]
#[derive(QueryableByName, Serialize)]
pub struct MovieCompact {
  pub movie_id: i32,
  pub title: String,
  pub description: String,
}
use crate::db::models::{Writer, Director, Artist};
use crate::chrono::NaiveDate;

#[derive(Serialize)]
pub struct SigninRes {
  pub token: String,
}

#[derive(Serialize)]
pub struct MovieRes {
  pub movie_id: i32,
  pub title: String,
  pub description: String,
  pub poster: String,
  pub genres: Vec<String>,
  pub languages: Vec<String>,
  pub release_country: String,
  #[serde(skip)]
  pub release_date: NaiveDate,
  pub duration: i16,
  pub writers: Vec<Writer>,
  pub directors: Vec<Director>,
  pub stars: Vec<Artist>,
}
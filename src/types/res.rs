use crate::db::models::{Writer, Director, Artist};

#[derive(Serialize)]
pub struct SigninRes {
  pub token: String,
}

#[drive(Serialize)]
pub struct MovieRes {
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
  writers: Vec<Writer>,
  directors: Vec<Director>,
  stars: Vec<Artist>,
}
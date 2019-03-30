use crate::chrono::NaiveDate;
use crate::db::models::{Movie, Artist, Director, Writer};

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

impl MovieRes {
    pub fn new(movie: Movie, writers: Vec<Writer>, directors: Vec<Director>, artists: Vec<Artist>) -> MovieRes {
        MovieRes {
            movie_id: movie.movie_id,
            title: movie.title,
            description: movie.description,
            poster: movie.poster,
            genres: movie.genres,
            languages: movie.languages,
            release_country: movie.release_country,
            release_date: movie.release_date,
            duration: movie.duration,
            writers,
            directors,
            stars: artists,
        }
    }
}
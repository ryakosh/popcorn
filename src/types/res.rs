use crate::chrono::NaiveDate;
use crate::db::models::{Artist, Director, Movie, Writer};

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
    pub release_date: NaiveDate,
    pub duration: i16,
    pub writers: Vec<String>,
    pub directors: Vec<String>,
    pub stars: Vec<String>,
    pub u: Option<MovieResU>,
}

#[derive(Serialize)]
pub struct MovieResU {
    pub rating: i16,
    pub is_watchlisted: bool,
    pub is_favorite: bool,
}

#[derive(Serialize)]
pub struct UserRatingRes {
    pub user_rating: i16,
}

impl MovieRes {
    pub fn new(
        movie: Movie,
        writers: Vec<Writer>,
        directors: Vec<Director>,
        artists: Vec<Artist>,
        u: Option<MovieResU>,
    ) -> MovieRes {
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
            writers: writers
                .into_iter()
                .map(|w| format!("{} {}", w.first_name, w.last_name))
                .collect(),
            directors: directors
                .into_iter()
                .map(|d| format!("{} {}", d.first_name, d.last_name))
                .collect(),
            stars: artists
                .into_iter()
                .map(|a| format!("{} {}", a.first_name, a.last_name))
                .collect(),
            u,
        }
    }
}

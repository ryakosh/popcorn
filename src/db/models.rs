use super::schema::{movies, users, ratings};
use crate::chrono::NaiveDate;

#[table_name = "users"]
#[derive(Queryable, Insertable, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub pwd: String,
}

#[derive(Queryable, Serialize)]
pub struct Artist {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Serialize)]
pub struct Director {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Serialize)]
pub struct Writer {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable)]
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
    pub score: f32,
}

#[table_name = "movies"]
#[derive(QueryableByName, Serialize)]
pub struct MovieCompact {
    pub movie_id: i32,
    pub title: String,
    pub release_date: NaiveDate,
    pub poster: String,
}

#[table_name = "ratings"]
#[derive(Insertable)]
pub struct NewRating {
    pub user_id: String,
    pub movie_id: i32,
    pub rating: i32,
}
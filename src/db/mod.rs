pub mod auth;
pub mod models;
pub mod movies_favorites;
pub mod movies_rate;
pub mod movies_watchlist;
pub mod schema;

use crate::diesel::{self, prelude::*};
use crate::error::Error;
use crate::filter::filter_movies;
use crate::types::{query::MoviesQuery, res::MovieResU};
use models::{Artist, Director, Movie, MovieCompact, Writer};
use schema::{
    artists, directors, movies, movies_artists, movies_directors, movies_writers, writers,
};
use std::env::var;

fn connect(conn_str: &str) -> PgConnection {
    PgConnection::establish(conn_str)
        .unwrap_or_else(|_| panic!("Error connecting to: {}", conn_str))
}

pub fn movies(movies_query: &MoviesQuery) -> Result<Vec<MovieCompact>, Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let movies_query = movies_query.validate()?;

    Ok(diesel::sql_query(movies_get_query(
        movies_query.search().unwrap_or(&"".to_string()),
        movies_query.limit().unwrap_or(10),
        movies_query.page().unwrap_or(1),
        &mut filter_movies(movies_query.filters().unwrap_or(&"".to_string())),
    ))
    .load(&conn)
    .expect("Error executing query"))
}

fn movies_get_query(search: &str, limit: i32, page: i32, filters: &mut String) -> String {
    if !search.is_empty() {
        if !filters.is_empty() {
            filters.insert_str(0, "AND ");
        }

        format!(
            include_str!("raw/movies-search.sql"),
            search,
            filters,
            limit,
            (page * limit) - limit
        )
    } else {
        if !filters.is_empty() {
            filters.insert_str(0, "WHERE ");
        }

        format!(
            include_str!("raw/movies.sql"),
            filters,
            limit,
            (page * limit) - limit
        )
    }
}

pub fn movie(
    id: i32,
    user_id: Option<&String>,
) -> Result<
    (
        Movie,
        Vec<Writer>,
        Vec<Director>,
        Vec<Artist>,
        Option<MovieResU>,
    ),
    Error,
> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let movie = movies::table.find(id).first(&conn);
    let ws = movies_writers::table
        .inner_join(writers::table.on(writers::writer_id.eq(movies_writers::writer_id)))
        .filter(movies_writers::movie_id.eq(id))
        .select((writers::first_name, writers::last_name))
        .load::<Writer>(&conn);
    let ds = movies_directors::table
        .inner_join(directors::table.on(directors::director_id.eq(movies_directors::director_id)))
        .filter(movies_directors::movie_id.eq(id))
        .select((directors::first_name, directors::last_name))
        .load::<Director>(&conn);
    let ats = movies_artists::table
        .inner_join(artists::table.on(artists::artist_id.eq(movies_artists::artist_id)))
        .filter(movies_artists::movie_id.eq(id))
        .select((artists::first_name, artists::last_name))
        .load::<Artist>(&conn);

    let mru = user_id.map(|user_id| {
        let rating = movies_rate::get_users_movie_rating(id, user_id);
        let is_watchlisted = movies_watchlist::is_movie_watchlisted(user_id, id);
        let is_favorite = movies_favorites::is_movie_favorite(user_id, id);

        MovieResU {
            rating,
            is_watchlisted,
            is_favorite,
        }
    });

    match (movie, ws, ds, ats) {
        (Ok(movie), Ok(ws), Ok(ds), Ok(ats)) => Ok((movie, ws, ds, ats, mru)),
        _ => Err(Error::NotFound),
    }
}

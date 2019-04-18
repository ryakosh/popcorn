pub mod models;
pub mod schema;

use crate::config::config;
use crate::diesel;
use crate::diesel::prelude::*;
use crate::diesel::sql_types::{Integer, Text, TinyInt};
use crate::error::*;
use crate::filter::filter_movies;
use crate::jsonwebtoken::{encode, Header};
use crate::types::data::{SigninData, SignupData};
use crate::types::query::MoviesQuery;
use crate::types::res::MovieRes;
use crate::types::Claims;
use models::{Artist, Director, Movie, MovieCompact, User, Writer};
use schema::{artists, directors, movies_artists, movies_directors, movies_writers, writers};
use std::env::var;

fn connect(conn_str: &str) -> PgConnection {
    PgConnection::establish(conn_str).expect(&format!("Error connecting to: {}", conn_str))
}

pub fn signup(singup_data: &SignupData) -> Result<(), Errors> {
    use schema::users;

    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let user: Result<User, _> = users::table
        .find(singup_data.uname().to_lowercase())
        .first(&conn);

    if let Ok(_) = user {
        Err(vec![Error::UnameTaken])
    } else {
        let new_user = User {
            id: singup_data.uname().to_string(),
            email: singup_data.email().to_string(),
            pwd: singup_data.pwd().to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&conn)
            .expect("Error creating a user");

        Ok(())
    }
}

pub fn signin(signin_data: &SigninData) -> Result<String, Errors> {
    use schema::users;

    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let result: Result<User, _> = users::table
        .find(signin_data.uname().to_lowercase())
        .first(&conn);

    if let Ok(user) = result {
        Ok(encode(
            &Header::default(),
            &Claims::new(user.id),
            &config.jwt.secret.as_ref(),
        )
        .expect("Error encoding token"))
    } else {
        Err(vec![Error::UserNFound])
    }
}

pub fn movies(movies_query: &MoviesQuery) -> Vec<MovieCompact> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let search = if let Some(search) = movies_query.search() {
        search
    } else {
        ""
    };
    let limit = movies_query.limit().unwrap_or(10);
    let page = movies_query.page().unwrap_or(1);
    let filters = if let Some(filters) = movies_query.filters() {
        filter_movies(filters).unwrap() // TODO: Return the `Error` to client
    } else {
        "".to_string()
    };

    let query = format!(
        include_str!("raw/movies.sql"),
        search,
        filters,
        limit,
        (page * limit) - limit
    );

    diesel::sql_query(query)
        .load(&conn)
        .expect("Error executing query")
}

pub fn movie(id: i32) -> Result<(Movie, Vec<Writer>, Vec<Director>, Vec<Artist>), Errors> {
    use schema::movies;

    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let movie = movies::table.find(id).first(&conn);
    let ws = movies_writers::table
        .inner_join(writers::table.on(writers::writer_id.eq(movies_writers::writer_id)))
        .filter(movies_writers::movie_id.eq(id))
        .select((
            writers::first_name,
            writers::last_name,
        ))
        .load::<Writer>(&conn);
    let ds = movies_directors::table
        .inner_join(directors::table.on(directors::director_id.eq(movies_directors::director_id)))
        .filter(movies_directors::movie_id.eq(id))
        .select((
            directors::first_name,
            directors::last_name,
        ))
        .load::<Director>(&conn);
    let ats = movies_artists::table
        .inner_join(artists::table.on(artists::artist_id.eq(movies_artists::artist_id)))
        .filter(movies_artists::movie_id.eq(id))
        .select((
            artists::first_name,
            artists::last_name,
        ))
        .load::<Artist>(&conn);

    match (movie, ws, ds, ats) {
        (Ok(movie), Ok(ws), Ok(ds), Ok(ats)) => Ok((movie, ws, ds, ats)),
        _ => Err(vec![Error::NotFound]),
    }
}

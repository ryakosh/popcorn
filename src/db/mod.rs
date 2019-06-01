pub mod models;
pub mod schema;

use crate::config::CONFIG;
use crate::diesel;
use crate::diesel::prelude::*;
use crate::error::*;
use crate::filter::filter_movies;
use crate::jsonwebtoken::{encode, Header};
use crate::types::data::{SigninData, SignupData};
use crate::types::query::MoviesQuery;
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
        if signin_data.pwd() == user.pwd {
            Ok(encode(
                &Header::default(),
                &Claims::new(user.id),
                &CONFIG.jwt.secret.as_ref(),
            )
            .expect("Error encoding token"))
        } else {
            Err(vec![Error::UserNFound])
        }
    } else {
        Err(vec![Error::UserNFound])
    }
}

pub fn movies(movies_query: &MoviesQuery) -> Result<Vec<MovieCompact>, Errors> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let mut errors = vec![];

    let limit = movies_query.limit().unwrap_or(10);
    let page = movies_query.page().unwrap_or(1);
    let mut filters = "".to_string();
    if let Some(qfilters) = movies_query.filters() {
        match filter_movies(qfilters) {
            Ok(qfilters) => { filters = qfilters; }
            Err(error) => { errors.push(error); }
        }
    } else {
        filters = "".to_string()
    };

    if errors.is_empty() {
        Ok(diesel::sql_query(movies_get_query(
            movies_query.search(),
            limit,
            page,
            &mut filters,
        ))
        .load(&conn)
        .expect("Error executing query"))
    } else {
        Err(errors)
    }
}

fn movies_get_query(
    moviesquery_search: Option<&String>,
    limit: i32,
    page: i32,
    filters: &mut String,
) -> String {
    if let Some(search) = moviesquery_search {
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

pub fn movie(id: i32) -> Result<(Movie, Vec<Writer>, Vec<Director>, Vec<Artist>), Errors> {
    use schema::movies;

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

    match (movie, ws, ds, ats) {
        (Ok(movie), Ok(ws), Ok(ds), Ok(ats)) => Ok((movie, ws, ds, ats)),
        _ => Err(vec![Error::NotFound]),
    }
}

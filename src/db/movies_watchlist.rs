use crate::db::schema::users_watchlist;
use crate::db::{auth::get_user_id, connect};
use crate::diesel::{self, prelude::*};
use crate::error::Error;
use std::env::var;

pub fn add_movie_to_watchlist(user_id: &str, movie_id: i32) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let result = users_watchlist::table
        .filter(users_watchlist::user_id.eq(user_id))
        .filter(users_watchlist::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    if let Ok(_) = result {
        Err(Error::EntryAlreadyExists)
    } else {
        diesel::insert_into(users_watchlist::table)
            .values(&(
                users_watchlist::user_id.eq(user_id),
                users_watchlist::movie_id.eq(movie_id),
            ))
            .execute(&conn)
            .expect("Error executing query");

        Ok(())
    }
}

pub fn delete_movie_from_watchlist(user_id: &str, movie_id: i32) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let result = users_watchlist::table
        .filter(users_watchlist::user_id.eq(user_id))
        .filter(users_watchlist::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    if let Ok(_) = result {
        diesel::delete(
            users_watchlist::table
                .filter(users_watchlist::user_id.eq(user_id))
                .filter(users_watchlist::movie_id.eq(movie_id)),
        )
        .execute(&conn)
        .expect("Error executing query");

        Ok(())
    } else {
        Err(Error::EntryDNExist)
    }
}

pub fn is_movie_watchlisted(user_id: &str, movie_id: i32) -> bool {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let result = users_watchlist::table
        .filter(users_watchlist::user_id.eq(user_id))
        .filter(users_watchlist::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    if let Ok(_) = result {
        true
    } else {
        false
    }
}

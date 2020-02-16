use crate::db::connect;
use crate::db::schema::users_favorites;
use crate::diesel::{self, prelude::*};
use crate::error::Error;
use std::env::var;

pub fn add_movie_to_favorites(user_id: &str, movie_id: i32) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    if result.is_ok() {
        Err(Error::EntryAlreadyExists)
    } else {
        diesel::insert_into(users_favorites::table)
            .values(&(
                users_favorites::user_id.eq(user_id),
                users_favorites::movie_id.eq(movie_id),
            ))
            .execute(&conn)
            .expect("Error executing query");

        Ok(())
    }
}

pub fn delete_movie_from_favorites(user_id: &str, movie_id: i32) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    if result.is_ok() {
        diesel::delete(
            users_favorites::table
                .filter(users_favorites::user_id.eq(user_id))
                .filter(users_favorites::movie_id.eq(movie_id)),
        )
        .execute(&conn)
        .expect("Error executing query");

        Ok(())
    } else {
        Err(Error::EntryDNExist)
    }
}

pub fn is_movie_favorite(user_id: &str, movie_id: i32) -> bool {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    result.is_ok()
}

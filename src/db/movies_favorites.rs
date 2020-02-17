use super::Conn;
use crate::db::schema::users_favorites;
use crate::diesel::{self, prelude::*};
use crate::error::Error;

pub fn add_movie_to_favorites(user_id: &str, movie_id: i32, conn: &Conn) -> Result<(), Error> {
    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(conn);

    if result.is_ok() {
        Err(Error::EntryAlreadyExists)
    } else {
        diesel::insert_into(users_favorites::table)
            .values(&(
                users_favorites::user_id.eq(user_id),
                users_favorites::movie_id.eq(movie_id),
            ))
            .execute(conn)
            .expect("Error executing query");

        Ok(())
    }
}

pub fn delete_movie_from_favorites(user_id: &str, movie_id: i32, conn: &Conn) -> Result<(), Error> {
    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(conn);

    if result.is_ok() {
        diesel::delete(
            users_favorites::table
                .filter(users_favorites::user_id.eq(user_id))
                .filter(users_favorites::movie_id.eq(movie_id)),
        )
        .execute(conn)
        .expect("Error executing query");

        Ok(())
    } else {
        Err(Error::EntryDNExist)
    }
}

pub fn is_movie_favorite(user_id: &str, movie_id: i32, conn: &Conn) -> bool {
    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(conn);

    result.is_ok()
}

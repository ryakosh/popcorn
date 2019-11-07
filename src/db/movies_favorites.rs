use crate::db::schema::users_favorites;
use crate::db::{auth::get_user_id, connect};
use crate::diesel::{self, prelude::*};
use crate::error::Error;
use crate::types::req_guards::ClaimedUser;
use std::env::var;

pub fn add_movie_to_favorites(cu: &ClaimedUser, movie_id: i32) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let user_id = get_user_id(&cu.uname(), &conn)?;
    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(&user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    if let Ok(_) = result {
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

pub fn delete_movie_from_favorites(cu: &ClaimedUser, movie_id: i32) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let user_id = get_user_id(&cu.uname(), &conn)?;

    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(&user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    if let Ok(_) = result {
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

pub fn is_movie_favorite(url_uname: &str, movie_id: i32) -> Result<bool, Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    let user_id = get_user_id(url_uname, &conn)?;
    let result = users_favorites::table
        .filter(users_favorites::user_id.eq(user_id))
        .filter(users_favorites::movie_id.eq(movie_id))
        .get_result::<(String, i32)>(&conn);

    if let Ok(_) = result {
        Ok(true)
    } else {
        Ok(false)
    }
}

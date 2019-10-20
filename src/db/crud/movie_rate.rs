use crate::db::connect;
use crate::db::models::NewUserRating;
use crate::db::schema::{movies, users, users_ratings};
use crate::diesel;
use crate::diesel::prelude::*;
use crate::diesel::result;
use crate::error::Error;
use crate::types::data::RateData;
use crate::types::req_guards::ClaimedUser;
use std::env::var;

pub fn create_movie_rate(
    movie_id: i32,
    claimed_user: &ClaimedUser,
    rate_data: &RateData,
) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    if let Ok(user_id) = users::table
        .find(claimed_user.uname())
        .select(users::id)
        .get_result::<String>(&conn)
    {
        let rate_data = rate_data.validate()?;

        conn.transaction::<_, result::Error, _>(|| {
            let user_rating = NewUserRating {
                user_id,
                movie_id,
                user_rating: rate_data.user_rating(),
            };
            let rc: (f32, i32) = movies::table // Rating and rating_count
                .find(movie_id)
                .select((movies::rating, movies::rating_count))
                .first(&conn)?;
            let new_rating =
                ((rc.0 * rc.1 as f32) + rate_data.user_rating() as f32) / (rc.1 + 1) as f32;
            diesel::insert_into(users_ratings::table)
                .values(&user_rating)
                .execute(&conn)?;

            diesel::update(movies::table.find(movie_id))
                .set((
                    movies::rating.eq(new_rating),
                    movies::rating_count.eq(rc.1 + 1),
                ))
                .execute(&conn)?;

            Ok(())
        })
        .expect("Error executing transaction");

        Ok(())
    } else {
        Err(Error::UserNFound)
    }
}

pub fn update_movie_rate(
    movie_id: i32,
    claimed_user: &ClaimedUser,
    rate_data: &RateData,
) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    if let Ok(user_id) = users::table
        .find(claimed_user.uname())
        .select(users::id)
        .get_result::<String>(&conn)
    {
        if let Ok(old_user_rating) = users_ratings::table
            .filter(users_ratings::user_id.eq(&user_id))
            .filter(users_ratings::movie_id.eq(movie_id))
            .select(users_ratings::user_rating)
            .get_result::<i16>(&conn)
        {
            let rate_data = rate_data.validate()?;

            conn.transaction::<_, result::Error, _>(|| {
                let rc: (f32, i32) = movies::table // Rating and rating_count
                    .find(movie_id)
                    .select((movies::rating, movies::rating_count))
                    .first(&conn)?;
                let new_rating = ((rc.0 * rc.1 as f32)
                    + (rate_data.user_rating() - old_user_rating) as f32)
                    / rc.1 as f32;

                diesel::update(
                    users_ratings::table
                        .filter(users_ratings::user_id.eq(user_id))
                        .filter(users_ratings::movie_id.eq(movie_id)),
                )
                .set(users_ratings::user_rating.eq(rate_data.user_rating()))
                .execute(&conn)?;

                diesel::update(movies::table.find(movie_id))
                    .set(movies::rating.eq(new_rating))
                    .execute(&conn)?;

                Ok(())
            })
            .expect("Error executing transaction");

            Ok(())
        } else {
            Err(Error::EntryDNExist)
        }
    } else {
        Err(Error::UserNFound)
    }
}

pub fn delete_movie_rate(movie_id: i32, claimed_user: &ClaimedUser) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    if let Ok(user_id) = users::table
        .find(claimed_user.uname())
        .select(users::id)
        .get_result::<String>(&conn)
    {
        if let Ok(user_rating) = users_ratings::table
            .filter(users_ratings::user_id.eq(&user_id))
            .filter(users_ratings::movie_id.eq(movie_id))
            .select(users_ratings::user_rating)
            .get_result::<i16>(&conn)
        {
            conn.transaction::<_, result::Error, _>(|| {
                let rc: (f32, i32) = movies::table // Rating and rating_count
                    .find(movie_id)
                    .select((movies::rating, movies::rating_count))
                    .first(&conn)?;
                let new_rating = ((rc.0 * rc.1 as f32) - user_rating as f32) / (rc.1 - 1) as f32;

                diesel::delete(
                    users_ratings::table
                        .filter(users_ratings::user_id.eq(user_id))
                        .filter(users_ratings::movie_id.eq(movie_id)),
                )
                .execute(&conn)?;

                diesel::update(movies::table.find(movie_id))
                    .set((
                        movies::rating.eq(new_rating),
                        movies::rating_count.eq(rc.1 - 1),
                    ))
                    .execute(&conn)?;

                Ok(())
            })
            .expect("Error executing transaction");

            Ok(())
        } else {
            Err(Error::EntryDNExist)
        }
    } else {
        Err(Error::UserNFound)
    }
}

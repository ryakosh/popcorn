use super::{connect, models::User, schema::users};
use crate::config::CONFIG;
use crate::diesel::{self, prelude::*};
use crate::error::Error;
use crate::jsonwebtoken::{encode, Header};
use crate::types::{
    data::{SigninData, SignupData},
    Claims,
    req_guards::ClaimedUser,
};
use std::env::var;

pub fn signup(signup_data: &SignupData) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let signup_data = signup_data.validate()?;

    let user: Result<User, _> = users::table
        .find(signup_data.uname().to_lowercase())
        .first(&conn);

    if let Ok(_) = user {
        Err(Error::UnameTaken)
    } else {
        let new_user = User {
            id: signup_data.uname().to_string(),
            email: signup_data.email().to_string(),
            pwd: signup_data.pwd().to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&conn)
            .expect("Error creating a user");

        Ok(())
    }
}

pub fn signin(signin_data: &SigninData) -> Result<String, Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let signin_data = signin_data.validate()?;

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
            Err(Error::UserNFound)
        }
    } else {
        Err(Error::UserNFound)
    }
}

pub fn get_user_id(claimed_user: &ClaimedUser, conn: &PgConnection) -> Result<String, Error> {    
    users::table
        .find(claimed_user.uname())
        .select(users::id)
        .get_result(conn)
        .map_err(|_| Error::UserNFound)
}
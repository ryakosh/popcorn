use super::{connect, models::User, schema::users};
use crate::argon2;
use crate::config::{self, CONFIG};
use crate::diesel::{self, prelude::*};
use crate::error::Error;
use crate::getrandom;
use crate::jsonwebtoken::{encode, Header};
use crate::types::{
    data::{SigninData, SignupData},
    Claims,
};
use std::env::var;

pub fn signup(signup_data: &SignupData) -> Result<(), Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));
    let signup_data = signup_data.validate()?;

    let user: Result<User, _> = users::table
        .find(signup_data.uname().to_lowercase())
        .first(&conn);

    if user.is_ok() {
        Err(Error::UnameTaken)
    } else {
        let new_user = User {
            id: signup_data.uname().to_string(),
            email: signup_data.email().to_string(),
            pwd: gen_pwd_hash(signup_data.pwd(), &CONFIG.argon2),
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
        if verify_pwd(signin_data.pwd(), &user.pwd) {
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

pub fn get_user_id(uname: &str) -> Result<String, Error> {
    let conn = connect(&var("DATABASE_URL").expect("Can't find DATABASE_URL environment variable"));

    users::table
        .find(uname)
        .select(users::id)
        .get_result(&conn)
        .map_err(|_| Error::UserNFound)
}

fn gen_pwd_hash(pwd: &str, config_argon2: &config::Argon2) -> String {
    let mut salt = vec![0u8; config_argon2.salt_length];
    let argon2_config = argon2::Config {
        variant: argon2::Variant::Argon2i,
        version: argon2::Version::Version13,
        mem_cost: config_argon2.mem_cost,
        time_cost: config_argon2.time_cost,
        lanes: config_argon2.lanes,
        thread_mode: argon2::ThreadMode::Parallel,
        hash_length: config_argon2.hash_length,
        ..argon2::Config::default()
    };

    getrandom::getrandom(&mut salt).expect("Error generating random salt");
    argon2::hash_encoded(pwd.as_bytes(), &salt, &argon2_config)
        .expect("Error generating password hash")
}

fn verify_pwd(pwd: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, pwd.as_bytes()).expect("Error verifying password")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pwd_hash_generation_works_correctly() {
        let test_pwd = "test_pwd123";
        let test_config = config::Argon2 {
            mem_cost: 100,
            time_cost: 1,
            lanes: 1,
            salt_length: 16,
            hash_length: 16,
        };

        let hash = gen_pwd_hash(test_pwd, &test_config);

        assert!(verify_pwd(test_pwd, &hash));
    }
}

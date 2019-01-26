mod schema;
mod models;

use std::env::var;
use crate::diesel::*;
use crate::types::data::SignupData;
use crate::error::*;
use schema::users::dsl::*;
use models::{User};

fn connect(conn_str: &str) -> PgConnection {
  PgConnection::establish(conn_str)
    .expect(&format!("Error connecting to: {}", conn_str))
}

pub fn signup(singup_data: &SignupData) -> Result<User, Errors> {
  let conn = connect(&var("DATABASE_URL")
    .expect("Can't find DATABASE_URL environment variable"));
  let user: Result<User, _> = users.find(singup_data.uname().to_lowercase()).first(&conn);

  if let Ok(_) = user {
    Err(vec![Error::UnameTaken])
  } else {
    let new_user = User {
      id: singup_data.uname().to_string(),
      email: singup_data.email().to_string(),
      pwd: singup_data.pwd().to_string(),
    };

    Ok(insert_into(users)
      .values(&new_user)
      .get_result(&conn)
      .expect("Error creating a user"))
  }
}
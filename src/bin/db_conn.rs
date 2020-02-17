use crate::rocket_contrib::databases::diesel;

#[database("popcorn")]
pub struct PopcornConn(diesel::PgConnection);
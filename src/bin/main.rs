#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;

fn main() {
  rocket
  ::ignite()
  .mount("/popcorn",
    routes![routes::signup,
            routes::signin,
            routes::movies,
            routes::movie,])
  .launch();
}
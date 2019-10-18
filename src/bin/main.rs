#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rocket::http::Method;
use rocket_cors::AllowedOrigins;

mod routes;

fn main() {
    let (allowed_origins, _) = AllowedOrigins::some(&["https://popcrn.ir"]);
    #[cfg(debug_assertions)]
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        ..Default::default()
    };

    rocket::ignite()
        .mount(
            "/popcorn",
            routes![
                routes::signup,
                routes::signin,
                routes::rate,
                routes::movies,
                routes::movie,
            ],
        )
        .attach(cors)
        .launch();
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rocket_cors::AllowedOrigins;

mod routes;

fn main() {
    let (allowed_origins, _) = AllowedOrigins::some(&["http://171.22.27.103"]);

    let cors = rocket_cors::Cors {
        allowed_origins,
        ..Default::default()
    };

    rocket::ignite()
        .mount(
            "/popcorn",
            routes![
                routes::signup,
                routes::signin,
                routes::movies,
                routes::movie,
            ],
        )
        .attach(cors)
        .launch();
}

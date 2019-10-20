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
                routes::auth::signup,
                routes::auth::signin,
                routes::movies,
                routes::movie,
                routes::movie_rate::get_user_rating,
                routes::movie_rate::create_movie_rate,
                routes::movie_rate::update_movie_rate,
                routes::movie_rate::delete_movie_rate,
            ],
        )
        .attach(cors)
        .launch();
}

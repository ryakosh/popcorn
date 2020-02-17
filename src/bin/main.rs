#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_cors;

use rocket::http::Method;
use rocket_cors::AllowedOrigins;

mod routes;
mod db_conn;

fn main() {
    #[cfg(not(debug_assertions))]
    let (allowed_origins, _) = AllowedOrigins::some(&["https://popcrn.ir"]);
    #[cfg(debug_assertions)]
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        ..Default::default()
    };

    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::auth::signup,
                routes::auth::signin,
                routes::movies,
                routes::movie,
                routes::movies_rate::get_users_movie_rating,
                routes::movies_rate::create_movie_rating,
                routes::movies_rate::update_movie_rating,
                routes::movies_rate::delete_movie_rating,
                routes::movies_watchlist::is_movie_watchlisted,
                routes::movies_watchlist::add_movie_to_watchlist,
                routes::movies_watchlist::delete_movie_from_watchlist,
                routes::movies_favorites::is_movie_favorite,
                routes::movies_favorites::add_movie_to_favorites,
                routes::movies_favorites::delete_movie_from_favorites,
            ],
        )
        .attach(cors)
        .attach(db_conn::PopcornConn::fairing())
        .launch();
}

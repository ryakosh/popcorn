use rocket_contrib::json::Json;
use rocket::request::Form;
use popcorn::types::Response;
use popcorn::types::data::{SignupData, SigninData};
use popcorn::types::query::MoviesQuery;
use popcorn::types::res::SigninRes;
use popcorn::db;
use popcorn::db::models::{User, MovieCompact, Movie};

#[post("/auth/signup", data = "<signup_data>", format = "json")]
pub fn signup(signup_data: Json<SignupData>)
  -> Json<Response<String>> {

  let result = db::signup(&signup_data.0);

  match result {
    Ok(()) => Json(Response::new()),
    Err(errors) => Json(Response::with_errors(errors)),
  }
}

#[post("/auth/signin", data = "<signin_data>", format = "json")]
pub fn signin(signin_data: Json<SigninData>)
  -> Json<Response<SigninRes>> {
  
  let result = db::signin(&signin_data.0);

  match result {
    Ok(token) => Json(Response::with_payload(SigninRes { token })),
    Err(errors) => Json(Response::with_errors(errors)),
  }
}

#[get("/movies?<movies_query..>")]
pub fn movies(movies_query: Form<MoviesQuery>)
  -> Json<Response<Vec<MovieCompact>>> {

  Json(Response::with_payload(db::movies(&movies_query.0)))
}

#[get("/movies/<id>")]
pub fn movie(id: i32) -> Json<Response<Movie>> {
  let result = db::movie(id);

  match result {
    Ok(movie) => Json(Response::with_payload(movie)),
    Err(errors) => Json(Response::with_errors(errors)),
  }
}
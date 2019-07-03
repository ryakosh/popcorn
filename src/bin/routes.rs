use popcorn::db;
use popcorn::db::models::MovieCompact;
use popcorn::types::data::{SigninData, SignupData, RateData};
use popcorn::types::query::MoviesQuery;
use popcorn::types::res::{MovieRes, SigninRes};
use popcorn::types::req_guards::ClaimedUser;
use popcorn::types::Response;
use rocket::request::Form;
use rocket_contrib::json::Json;

#[post("/auth/signup", data = "<signup_data>", format = "json")]
pub fn signup(signup_data: Json<SignupData>) -> Json<Response<String>> {
    let result = db::signup(&signup_data.0);

    match result {
        Ok(()) => Json(Response::new()),
        Err(error) => Json(Response::with_error(error)),
    }
}

#[post("/auth/signin", data = "<signin_data>", format = "json")]
pub fn signin(signin_data: Json<SigninData>) -> Json<Response<SigninRes>> {
    let result = db::signin(&signin_data.0);

    match result {
        Ok(token) => Json(Response::with_payload(SigninRes { token })),
        Err(error) => Json(Response::with_error(error)),
    }
}

#[post("/movies/<id>/rate", data = "<rate_data>", format = "json")]
pub fn rate(claimed_user: ClaimedUser, id: i32, rate_data: Json<RateData>) ->
    Json<Response<String>> {
    
    let result = db::rate(&claimed_user, id, &rate_data.0);

    match result {
        Ok(()) => Json(Response::new()),
        Err(error) => Json(Response::with_error(error)),
    }
}

#[get("/movies?<movies_query..>")]
pub fn movies(movies_query: Form<MoviesQuery>) -> Json<Response<Vec<MovieCompact>>> {
    match db::movies(&movies_query.0) {
        Ok(result) => Json(Response::with_payload(result)),
        Err(error) => Json(Response::with_error(error)),
    }
}

#[get("/movies/<id>")]
pub fn movie(id: i32) -> Json<Response<MovieRes>> {
    let result = db::movie(id);

    match result {
        Ok(result) => Json(Response::with_payload(MovieRes::new(
            result.0, result.1, result.2, result.3,
        ))),
        Err(error) => Json(Response::with_error(error)),
    }
}

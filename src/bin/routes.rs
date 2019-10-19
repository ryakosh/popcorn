use popcorn::db;
use popcorn::db::models::MovieCompact;
use popcorn::types::data::{RateData, SigninData, SignupData};
use popcorn::types::query::MoviesQuery;
use popcorn::types::req_guards::ClaimedUser;
use popcorn::types::res::{MovieRes, SigninRes};
use popcorn::types::Response;
use rocket::request::Form;
use rocket::response::status;
use rocket_contrib::json::Json;

#[post("/auth/signup", data = "<signup_data>", format = "json")]
pub fn signup(
    signup_data: Json<SignupData>,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = db::signup(&signup_data.0);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/auth/signin", data = "<signin_data>", format = "json")]
pub fn signin(
    signin_data: Json<SigninData>,
) -> Result<Json<Response<SigninRes>>, status::BadRequest<Json<Response<String>>>> {
    let result = db::signin(&signin_data.0);

    match result {
        Ok(token) => Ok(Json(Response::with_payload(SigninRes { token }))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[get("/movies?<movies_query..>")]
pub fn movies(
    movies_query: Form<MoviesQuery>,
) -> Result<Json<Response<Vec<MovieCompact>>>, status::BadRequest<Json<Response<String>>>> {
    match db::movies(&movies_query.0) {
        Ok(result) => Ok(Json(Response::with_payload(result))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[get("/movies/<id>")]
pub fn movie(
    id: i32,
) -> Result<Json<Response<MovieRes>>, status::BadRequest<Json<Response<String>>>> {
    let result = db::movie(id);

    match result {
        Ok(result) => Ok(Json(Response::with_payload(MovieRes::new(
            result.0, result.1, result.2, result.3,
        )))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/movies/<id>/rate", data = "<rate_data>", format = "json")]
pub fn movie_rate(
    claimed_user: ClaimedUser,
    id: i32,
    rate_data: Json<RateData>,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = db::movie_rate(id, &claimed_user, &rate_data);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

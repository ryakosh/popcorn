use popcorn::db::{self, auth, crud, models::MovieCompact};
use popcorn::types::data::{RateData, SigninData, SignupData};
use popcorn::types::res::{MovieRes, SigninRes, UserRatingRes};
use popcorn::types::{query::MoviesQuery, req_guards::ClaimedUser, Response};
use rocket::{request::Form, response::status};
use rocket_contrib::json::Json;

#[post("/auth/signup", data = "<signup_data>", format = "json")]
pub fn signup(
    signup_data: Json<SignupData>,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = auth::signup(&signup_data.0);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/auth/signin", data = "<signin_data>", format = "json")]
pub fn signin(
    signin_data: Json<SigninData>,
) -> Result<Json<Response<SigninRes>>, status::BadRequest<Json<Response<String>>>> {
    let result = auth::signin(&signin_data.0);

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

#[get("/movies/<id>/rate")]
pub fn get_user_rating(
    claimed_user: ClaimedUser,
    id: i32,
) -> Result<Json<Response<UserRatingRes>>, status::BadRequest<Json<Response<String>>>> {
    let result = crud::movie_rate::get_user_rating(id, &claimed_user);

    match result {
        Ok(user_rating) => Ok(Json(Response::with_payload(UserRatingRes { user_rating }))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/movies/<id>/rate", data = "<rate_data>", format = "json")]
pub fn create_movie_rate(
    claimed_user: ClaimedUser,
    id: i32,
    rate_data: Json<RateData>,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = crud::movie_rate::create_movie_rate(id, &claimed_user, &rate_data);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[put("/movies/<id>/rate", data = "<rate_data>", format = "json")]
pub fn update_movie_rate(
    claimed_user: ClaimedUser,
    id: i32,
    rate_data: Json<RateData>,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = crud::movie_rate::update_movie_rate(id, &claimed_user, &rate_data);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/movies/<id>/rate")]
pub fn delete_movie_rate(
    claimed_user: ClaimedUser,
    id: i32,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = crud::movie_rate::delete_movie_rate(id, &claimed_user);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

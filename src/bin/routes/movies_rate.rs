use popcorn::db::{auth::get_user_id, movies_rate};
use popcorn::types::{data::RateData, req_guards::ClaimedUser, res::UsersMovieRatingRes, Response};
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db_conn::PopcornConn;

#[get("/users/<uname>/movies/<id>?rate", format = "json", rank = 3)]
pub fn get_users_movie_rating(
    uname: String,
    id: i32,
    conn: PopcornConn,
) -> Result<Json<Response<UsersMovieRatingRes>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(&uname, &conn.0) {
        Ok(user_id) => {
            let user_rating = movies_rate::get_users_movie_rating(id, &user_id, &conn.0);

            Ok(Json(Response::with_payload(UsersMovieRatingRes {
                user_rating,
            })))
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post(
    "/users/<_uname>/movies/<id>?rate",
    data = "<rate_data>",
    format = "json",
    rank = 3
)]
pub fn create_movie_rating(
    cu: ClaimedUser,
    id: i32,
    rate_data: Json<RateData>,
    _uname: String,
    conn: PopcornConn,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname(), &conn.0) {
        Ok(user_id) => {
            let result = movies_rate::create_movie_rating(id, &user_id, &rate_data, &conn.0);

            match result {
                Ok(()) => Ok(Json(Response::default())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[put(
    "/users/<_uname>/movies/<id>?rate",
    data = "<rate_data>",
    format = "json",
    rank = 3
)]
pub fn update_movie_rating(
    cu: ClaimedUser,
    id: i32,
    rate_data: Json<RateData>,
    _uname: String,
    conn: PopcornConn,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname(), &conn.0) {
        Ok(user_id) => {
            let result = movies_rate::update_movie_rating(id, &user_id, &rate_data, &conn.0);

            match result {
                Ok(()) => Ok(Json(Response::default())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/users/<_uname>/movies/<id>?rate", rank = 3)]
pub fn delete_movie_rating(
    cu: ClaimedUser,
    id: i32,
    _uname: String,
    conn: PopcornConn,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname(), &conn.0) {
        Ok(user_id) => {
            let result = movies_rate::delete_movie_rating(id, &user_id, &conn.0);

            match result {
                Ok(()) => Ok(Json(Response::default())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

use popcorn::db::{auth::get_user_id, movies_rate};
use popcorn::types::{data::RateData, req_guards::ClaimedUser, res::UsersMovieRatingRes, Response};
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/users/<_uname>/movies/<id>?rate", format = "json", rank = 3)]
pub fn get_users_movie_rating(
    cu: ClaimedUser,
    id: i32,
    _uname: String,
) -> Result<Json<Response<UsersMovieRatingRes>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movies_rate::get_users_movie_rating(id, &user_id);

            match result {
                Ok(user_rating) => Ok(Json(Response::with_payload(UsersMovieRatingRes {
                    user_rating,
                }))),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
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
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movies_rate::create_movie_rating(id, &user_id, &rate_data);

            match result {
                Ok(()) => Ok(Json(Response::new())),
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
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movies_rate::update_movie_rating(id, &user_id, &rate_data);

            match result {
                Ok(()) => Ok(Json(Response::new())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/users/<_uname>/movies/<id>?rate", format = "json", rank = 3)]
pub fn delete_movie_rating(
    cu: ClaimedUser,
    id: i32,
    _uname: String,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movies_rate::delete_movie_rating(id, &user_id);

            match result {
                Ok(()) => Ok(Json(Response::new())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

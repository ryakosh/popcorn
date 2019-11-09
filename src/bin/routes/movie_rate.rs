use popcorn::db::{auth::get_user_id, movie_rate};
use popcorn::types::{data::RateData, req_guards::ClaimedUser, res::UserRatingRes, Response};
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/movies/<id>/rate")]
pub fn get_user_rating(
    cu: ClaimedUser,
    id: i32,
) -> Result<Json<Response<UserRatingRes>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movie_rate::get_user_rating(id, &user_id);

            match result {
                Ok(user_rating) => Ok(Json(Response::with_payload(UserRatingRes { user_rating }))),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/movies/<id>/rate", data = "<rate_data>", format = "json")]
pub fn create_movie_rate(
    cu: ClaimedUser,
    id: i32,
    rate_data: Json<RateData>,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movie_rate::create_movie_rate(id, &user_id, &rate_data);

            match result {
                Ok(()) => Ok(Json(Response::new())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[put("/movies/<id>/rate", data = "<rate_data>", format = "json")]
pub fn update_movie_rate(
    cu: ClaimedUser,
    id: i32,
    rate_data: Json<RateData>,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movie_rate::update_movie_rate(id, &user_id, &rate_data);

            match result {
                Ok(()) => Ok(Json(Response::new())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/movies/<id>/rate")]
pub fn delete_movie_rate(
    cu: ClaimedUser,
    id: i32,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movie_rate::delete_movie_rate(id, &user_id);

            match result {
                Ok(()) => Ok(Json(Response::new())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

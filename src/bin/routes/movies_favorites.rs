use popcorn::db::{auth::get_user_id, movies_favorites};
use popcorn::types::{req_guards::ClaimedUser, Response};
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

#[get("/users/<uname>/movies/<id>?favorite", format = "json", rank = 2)]
pub fn is_movie_favorite(
    uname: String,
    id: i32,
) -> Result<Json<Response<JsonValue>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(&uname) {
        Ok(user_id) => {
            let is_favorite = movies_favorites::is_movie_favorite(&user_id, id);
            Ok(Json(Response::with_payload(json!({
                "isFavorite": is_favorite
            }))))
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/users/<uname>/movies/<id>?favorite", format = "json", rank = 2)]
pub fn add_movie_to_favorites(
    cu: ClaimedUser,
    uname: String,
    id: i32,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movies_favorites::add_movie_to_favorites(&user_id, id);

            match result {
                Ok(()) => Ok(Json(Response::new())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/users/<uname>/movies/<id>?favorite", format = "json", rank = 2)]
pub fn delete_movie_from_favorites(
    cu: ClaimedUser,
    uname: String,
    id: i32,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname()) {
        Ok(user_id) => {
            let result = movies_favorites::delete_movie_from_favorites(&user_id, id);

            match result {
                Ok(()) => Ok(Json(Response::new())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

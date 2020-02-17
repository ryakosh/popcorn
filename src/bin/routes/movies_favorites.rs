use popcorn::db::{auth::get_user_id, movies_favorites};
use popcorn::types::{req_guards::ClaimedUser, Response};
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

use crate::db_conn::PopcornConn;

#[get("/users/<uname>/movies/<id>?favorite", format = "json", rank = 2)]
pub fn is_movie_favorite(
    uname: String,
    id: i32,
    conn: PopcornConn,
) -> Result<Json<Response<JsonValue>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(&uname, &conn.0) {
        Ok(user_id) => {
            let is_favorite = movies_favorites::is_movie_favorite(&user_id, id, &conn.0);
            Ok(Json(Response::with_payload(json!({
                "isFavorite": is_favorite
            }))))
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/users/<_uname>/movies/<id>?favorite", rank = 2)]
pub fn add_movie_to_favorites(
    cu: ClaimedUser,
    _uname: String,
    id: i32,
    conn: PopcornConn,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname(), &conn.0) {
        Ok(user_id) => {
            let result = movies_favorites::add_movie_to_favorites(&user_id, id, &conn.0);

            match result {
                Ok(()) => Ok(Json(Response::default())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/users/<_uname>/movies/<id>?favorite", rank = 2)]
pub fn delete_movie_from_favorites(
    cu: ClaimedUser,
    _uname: String,
    id: i32,
    conn: PopcornConn,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname(), &conn.0) {
        Ok(user_id) => {
            let result = movies_favorites::delete_movie_from_favorites(&user_id, id, &conn.0);

            match result {
                Ok(()) => Ok(Json(Response::default())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

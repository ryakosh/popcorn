use popcorn::db::{auth::get_user_id, movies_watchlist};
use popcorn::types::{req_guards::ClaimedUser, Response};
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

use crate::db_conn::PopcornConn;

#[get("/users/<uname>/movies/<id>?watchlist", format = "json", rank = 1)]
pub fn is_movie_watchlisted(
    uname: String,
    id: i32,
    conn: PopcornConn,
) -> Result<Json<Response<JsonValue>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(&uname, &conn.0) {
        Ok(user_id) => {
            let is_watchlisted = movies_watchlist::is_movie_watchlisted(&user_id, id, &conn.0);
            Ok(Json(Response::with_payload(json!({
                "isWatchlisted": is_watchlisted
            }))))
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/users/<_uname>/movies/<id>?watchlist", rank = 1)]
pub fn add_movie_to_watchlist(
    cu: ClaimedUser,
    _uname: String,
    id: i32,
    conn: PopcornConn,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname(), &conn.0) {
        Ok(user_id) => {
            let result = movies_watchlist::add_movie_to_watchlist(&user_id, id, &conn.0);

            match result {
                Ok(()) => Ok(Json(Response::default())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/users/<_uname>/movies/<id>?watchlist", rank = 1)]
pub fn delete_movie_from_watchlist(
    cu: ClaimedUser,
    _uname: String,
    id: i32,
    conn: PopcornConn,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    match get_user_id(cu.uname(), &conn.0) {
        Ok(user_id) => {
            let result = movies_watchlist::delete_movie_from_watchlist(&user_id, id, &conn.0);

            match result {
                Ok(()) => Ok(Json(Response::default())),
                Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
            }
        }
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

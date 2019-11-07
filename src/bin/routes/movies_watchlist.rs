use popcorn::db::movies_watchlist;
use popcorn::types::{req_guards::ClaimedUser, Response};
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

#[get("/users/<uname>/movies/<id>?watchlist", format = "json")]
pub fn is_movie_watchlisted(
    uname: String,
    id: i32,
) -> Result<Json<Response<JsonValue>>, status::BadRequest<Json<Response<String>>>> {
    let result = movies_watchlist::is_movie_watchlisted(&uname, id);

    match result {
        Ok(is_watchlisted) => Ok(Json(Response::with_payload(json!({
            "isWatchlisted": is_watchlisted
        })))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/users/<uname>/movies/<id>?watchlist", format = "json")]
pub fn add_movie_to_watchlist(
    cu: ClaimedUser,
    uname: String,
    id: i32,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = movies_watchlist::add_movie_to_watchlist(&cu, id);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/users/<uname>/movies/<id>?watchlist", format = "json")]
pub fn delete_movie_from_watchlist(
    cu: ClaimedUser,
    uname: String,
    id: i32,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = movies_watchlist::delete_movie_from_watchlist(&cu, id);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

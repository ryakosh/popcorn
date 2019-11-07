use popcorn::db::movies_favorites;
use popcorn::types::{req_guards::ClaimedUser, Response};
use rocket::response::status;
use rocket_contrib::json::{Json, JsonValue};

#[get("/users/<uname>/movies/<id>?favorite", format = "json", rank = 2)]
pub fn is_movie_favorite(
    uname: String,
    id: i32,
) -> Result<Json<Response<JsonValue>>, status::BadRequest<Json<Response<String>>>> {
    let result = movies_favorites::is_movie_favorite(&uname, id);

    match result {
        Ok(is_favorite) => Ok(Json(Response::with_payload(json!({
            "isFavorite": is_favorite
        })))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/users/<uname>/movies/<id>?favorite", format = "json", rank = 2)]
pub fn add_movie_to_favorites(
    cu: ClaimedUser,
    uname: String,
    id: i32,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = movies_favorites::add_movie_to_favorites(&cu, id);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[delete("/users/<uname>/movies/<id>?favorite", format = "json", rank = 2)]
pub fn delete_movie_from_favorites(
    cu: ClaimedUser,
    uname: String,
    id: i32,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = movies_favorites::delete_movie_from_favorites(&cu, id);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

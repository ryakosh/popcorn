pub mod auth;
pub mod movies_favorites;
pub mod movies_rate;
pub mod movies_watchlist;

use popcorn::db::{self, auth::get_user_id, models::MovieCompact};
use popcorn::types::req_guards::ClaimedUser;
use popcorn::types::res::MovieRes;
use popcorn::types::{query::MoviesQuery, Response};
use rocket::{request::Form, response::status};
use rocket_contrib::json::Json;

use crate::db_conn::PopcornConn;

#[get("/movies?<movies_query..>", format = "json")]
pub fn movies(
    movies_query: Form<MoviesQuery>,
    conn: PopcornConn,
) -> Result<Json<Response<Vec<MovieCompact>>>, status::BadRequest<Json<Response<String>>>> {
    match db::movies(&movies_query.0, &conn.0) {
        Ok(result) => Ok(Json(Response::with_payload(result))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[get("/movies/<id>", format = "json")]
pub fn movie(
    id: i32,
    cu: Option<ClaimedUser>,
    conn: PopcornConn,
) -> Result<Json<Response<MovieRes>>, status::BadRequest<Json<Response<String>>>> {
    let user_id = if let Some(cu) = cu {
        match get_user_id(cu.uname(), &conn.0) {
            Ok(user_id) => Some(user_id),
            Err(error) => return Err(status::BadRequest(Some(Json(Response::with_error(error))))),
        }
    } else {
        None
    };
    let result = db::movie(id, user_id.as_ref(), &conn.0);

    match result {
        Ok(result) => Ok(Json(Response::with_payload(MovieRes::new(
            result.0, result.1, result.2, result.3, result.4,
        )))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

pub mod auth;
pub mod movie_rate;

use popcorn::db::{self, models::MovieCompact};
use popcorn::types::res::MovieRes;
use popcorn::types::{query::MoviesQuery, Response};
use rocket::{request::Form, response::status};
use rocket_contrib::json::Json;

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

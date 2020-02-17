use popcorn::db::auth;
use popcorn::types::{
    data::{SigninData, SignupData},
    res::SigninRes,
    Response,
};
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db_conn::PopcornConn;

#[post("/auth/signup", data = "<signup_data>", format = "json")]
pub fn signup(
    signup_data: Json<SignupData>,
    conn: PopcornConn,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = auth::signup(&signup_data.0, &conn.0);

    match result {
        Ok(()) => Ok(Json(Response::default())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/auth/signin", data = "<signin_data>", format = "json")]
pub fn signin(
    signin_data: Json<SigninData>,
    conn: PopcornConn,
) -> Result<Json<Response<SigninRes>>, status::BadRequest<Json<Response<String>>>> {
    let result = auth::signin(&signin_data.0, &conn.0);

    match result {
        Ok(token) => Ok(Json(Response::with_payload(SigninRes { token }))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

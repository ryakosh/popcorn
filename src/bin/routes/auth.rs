use popcorn::db::auth;
use popcorn::types::{
    data::{SigninData, SignupData},
    res::SigninRes,
    Response,
};
use rocket::response::status;
use rocket_contrib::json::Json;

#[post("/auth/signup", data = "<signup_data>", format = "json")]
pub fn signup(
    signup_data: Json<SignupData>,
) -> Result<Json<Response<String>>, status::BadRequest<Json<Response<String>>>> {
    let result = auth::signup(&signup_data.0);

    match result {
        Ok(()) => Ok(Json(Response::new())),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

#[post("/auth/signin", data = "<signin_data>", format = "json")]
pub fn signin(
    signin_data: Json<SigninData>,
) -> Result<Json<Response<SigninRes>>, status::BadRequest<Json<Response<String>>>> {
    let result = auth::signin(&signin_data.0);

    match result {
        Ok(token) => Ok(Json(Response::with_payload(SigninRes { token }))),
        Err(error) => Err(status::BadRequest(Some(Json(Response::with_error(error))))),
    }
}

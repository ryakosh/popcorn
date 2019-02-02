use rocket_contrib::json::Json;
use popcorn::types::Response;
use popcorn::types::data::SignupData;
use popcorn::db;
use popcorn::db::models::User;

#[post("/auth/signup", data = "<signup_data>", format = "json")]
pub fn signup(signup_data: Json<SignupData>) -> Json<Response<User>> {
  let result = db::signup(&signup_data.0);

  match result {
    Ok(user) => Json(Response::with_payload(user)),
    Err(errors) => Json(Response::with_errors(errors)),
  }
}

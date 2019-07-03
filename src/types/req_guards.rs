use crate::rocket;
use crate::rocket::http;
use crate::rocket::request;
use crate::jsonwebtoken::{decode, Validation, Algorithm};

use crate::types;
use crate::config::CONFIG;
use crate::error;

#[derive(Debug)]
pub struct ClaimedUser(String);

impl ClaimedUser {
    pub fn uname(&self) -> &str {
        &self.0
    }
}

impl<'a, 'r> request::FromRequest<'a, 'r> for ClaimedUser {
    type Error = error::Error;

    fn from_request(request: &'a request::Request<'r>) ->
        request::Outcome<Self, Self::Error> {

        let authorization = request.headers().get_one("Authorization");

        if let Some(authorization) = authorization {
            let authorization: Vec<_> =
                authorization.trim().splitn(3, ' ').take(2).collect();
            
            if authorization.len() == 2 {
                if authorization[0] == "Bearer" {
                    let result = decode::<types::Claims>(
                        authorization[1],
                        &CONFIG.jwt.secret.as_ref(),
                        &Validation::new(Algorithm::HS256));
                    
                    match result {
                        Ok(token) => {
                            rocket::Outcome::Success(
                                ClaimedUser(token.claims.sub)
                            )
                        },
                        Err(_) => {
                            rocket::Outcome::Failure((
                                http::Status::Unauthorized,
                                error::Error::TokenInvalid,
                            ))
                        }
                    }
                } else {
                    rocket::Outcome::Failure((http::Status::Unauthorized,
                        error::Error::AuthorizationInvalid))
                }
            } else {
                rocket::Outcome::Failure((http::Status::Unauthorized,
                    error::Error::AuthorizationInvalid))
            }
        } else {
            rocket::Outcome::Failure((http::Status::Unauthorized,
                error::Error::AuthRequired))
        }
    }
}
use crate::error::Error;
use crate::serde::Serialize;
use std::time::{Duration, SystemTime};

pub mod data;
pub mod query;
pub mod res;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    iat: u64,
    sub: String,
    exp: u64,
}

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub payload: Option<T>,
    pub error: Option<Error>,
}

impl Claims {
    pub fn new(sub: String) -> Claims {
        Claims {
            iss: "Popcorn".to_string(),
            iat: Self::gen_iat(),
            exp: Self::gen_exp(),
            sub,
        }
    }

    fn gen_exp() -> u64 {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH");
        (now + Duration::from_secs(3 * 3600)).as_secs()
    }

    fn gen_iat() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH")
            .as_secs()
    }
}

impl<T: Serialize> Response<T> {
    pub fn new() -> Self {
        Response {
            payload: None,
            error: None,
        }
    }

    pub fn with_payload(payload: T) -> Self {
        Response {
            payload: Some(payload),
            error: None,
        }
    }

    pub fn with_error(error: Error) -> Self {
        Response {
            payload: None,
            error: Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claims_gen_exp_is_valid() {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH")
            .as_secs();
        let exp = Claims::gen_exp();

        assert!(exp > now);
    }

    #[test]
    fn claims_gen_iat_is_valid() {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH")
            .as_secs();
        let iat = Claims::gen_iat();

        assert!(iat == now);
    }
}

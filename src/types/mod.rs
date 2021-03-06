use crate::error::Error;
use crate::serde::Serialize;
use std::time::{Duration, SystemTime};

pub mod data;
pub mod query;
pub mod req_guards;
pub mod res;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub iat: u64,
    pub sub: String,
    pub exp: u64,
}

#[derive(Serialize, Debug)]
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

    #[test]
    fn response_initializers_work_correctly() {
        let res: Response<()> = Response::new();
        assert_eq!(res.payload, None);
        assert_eq!(res.error, None);

        let test_payload = "test";
        let res = Response::with_payload(test_payload);
        assert_eq!(res.payload, Some(test_payload));
        assert_eq!(res.error, None);

        let res: Response<()> = Response::with_error(Error::NotFound);
        assert_eq!(res.payload, None);
        assert_eq!(res.error, Some(Error::NotFound));
    }
}

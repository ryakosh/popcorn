use std::time::{Duration, SystemTime};

pub mod data;

#[derive(Serialize, Deserialize)]
pub struct Claims {
  iss: String,
  iat: u64,
  sub: String,
  exp: u64,
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
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
      .expect("SystemTime before UNIX EPOCH");
    (now + Duration::from_secs(3 * 3600)).as_secs()
  }

  fn gen_iat() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
      .expect("SystemTime before UNIX EPOCH")
      .as_secs()
  }
}
use std::fs::read_to_string;
use crate::toml::from_str;

lazy_static! {
  pub static ref config: Config = Config::new();
}

#[derive(Deserialize)]
pub struct Config {
  pub jwt: JWT,
}

#[derive(Deserialize)]
pub struct JWT {
  pub secret: String,
}

impl Config {
  fn new() -> Self {
    from_str(&read_to_string("./popcorn.toml").expect("Error reading ./popcorn.toml"))
      .expect("Error deserializing the config file")
  }
}
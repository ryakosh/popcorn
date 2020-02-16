use crate::toml::from_str;
use std::fs::read_to_string;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Deserialize)]
pub struct Config {
    pub jwt: JWT,
    pub argon2: Argon2,
}

#[derive(Deserialize)]
pub struct JWT {
    pub secret: String,
}

#[derive(Deserialize)]
pub struct Argon2 {
    pub mem_cost: u32,
    pub time_cost: u32,
    pub lanes: u32,
    pub salt_length: usize,
    pub hash_length: u32,
}

impl Config {
    fn new() -> Self {
        from_str(&read_to_string("./popcorn.toml").expect("Error reading ./popcorn.toml"))
            .expect("Error deserializing the config file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_fields_deserialize_correctly() {
        let test_secret = "test";
        let test_config: Config = from_str(&format!("[jwt]\nsecret=\"{}\"", test_secret))
            .expect("Error deserializing the config file");

        assert_eq!(test_secret, test_config.jwt.secret);
    }
}

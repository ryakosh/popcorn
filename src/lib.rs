#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use] extern crate diesel;
extern crate chrono;

pub mod db;
pub mod error;
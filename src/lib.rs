#![feature(uniform_paths)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
extern crate chrono;
extern crate regex;

pub mod db;
pub mod error;
pub mod types;
pub mod consts;
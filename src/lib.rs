#![feature(uniform_paths)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
extern crate chrono;
extern crate regex;
extern crate jsonwebtoken;
extern crate toml;
extern crate serde;

pub mod db;
pub mod error;
pub mod types;
pub mod consts;
pub mod config;
pub mod filter;
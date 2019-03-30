#![feature(uniform_paths)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate jsonwebtoken;
extern crate regex;
extern crate serde;
extern crate toml;

pub mod config;
pub mod consts;
pub mod db;
pub mod error;
pub mod filter;
pub mod types;

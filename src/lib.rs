#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;
extern crate regex;
extern crate reqwest;
extern crate log;
extern crate env_logger;
extern crate rand;

pub mod router;
pub mod database;
pub mod utils;
pub mod constants;

#[cfg(test)]
pub mod tests;

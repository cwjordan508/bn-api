#![deny(unreachable_patterns)]
#![deny(unknown_lints)]
#![deny(unused_variables)]
#![deny(unused_imports)]
// Unused results is more often than not an error
#![deny(unused_must_use)]
//#![deny(unused_extern_crates)]
extern crate actix_web;
extern crate bigneon_db;
extern crate diesel;
extern crate dotenv;
//extern crate futures;
//extern crate http;
//extern crate hyper;
//extern crate hyper_tls;
extern crate itertools;
extern crate lettre;
extern crate lettre_email;
extern crate scheduled_thread_pool;

//extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate crypto;
extern crate jwt;
extern crate serde_with;
extern crate tari_client;
//extern crate url;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate log4rs;
extern crate reqwest;
extern crate rustc_serialize;
extern crate stripe;
#[macro_use]
extern crate validator_derive;
extern crate validator;

pub mod auth;
pub mod config;
pub mod controllers;
pub mod db;
pub mod errors;
pub mod helpers;
pub mod mail;
pub mod middleware;
pub mod models;
mod payments;
mod routing;
pub mod server;
pub mod utils;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate reqwest;

mod requester;
mod client;
mod error;
pub mod model;

pub use error::Error;
pub use client::Client;

pub const BASE_URL: &'static str = "https://www.wanikani.com/api/v2";

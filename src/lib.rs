#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate chrono;
extern crate reqwest;

mod test_helpers;

mod requester;
mod client;
mod error;
pub mod model;
pub mod filters;

pub use error::Error;
pub use error::Result;
pub use client::Client;

pub const BASE_URL: &'static str = "https://api.wanikani.com/v2";

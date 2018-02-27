#![cfg(test)]

use std::fs::File;
use chrono::prelude::{DateTime, Utc};

pub fn date(datestr: &str) -> DateTime<Utc> {
    datestr.parse::<DateTime<Utc>>().unwrap()
}

pub fn maybe_date(datestr: &str) -> Option<DateTime<Utc>> {
    match datestr.parse::<DateTime<Utc>>() {
        Err(_) => None,
        Ok(date) => Some(date),
    }
}

pub fn fixture(name: &str) -> File {
    let filename = format!("./tests/fixtures/{}.json", name);
    File::open(filename).unwrap()
}

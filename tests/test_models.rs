extern crate wanikani;
extern crate serde_json;
extern crate chrono;

mod helpers;

use helpers::fixture;
use helpers::generators::*;
use wanikani::model::*;

#[test]
fn parse_user() {
    let file = fixture("user");
    let user: Report<User> = serde_json::from_reader(file).unwrap();

    assert!(user == gen_user());
}

#[test]
fn parse_subject() {
    let file = fixture("subject");
    let subject: Report<Subject> = serde_json::from_reader(file).unwrap();

    assert!(subject == gen_subject());
}

#[test]
fn parse_assignment() {
    let file = fixture("assignment");
    let assignment: Report<Assignment> = serde_json::from_reader(file).unwrap();

    assert!(assignment == gen_assignment());
}

use std::fs::File;

pub fn fixture(name: &str) -> File {
    let filename = format!("./tests/fixtures/{}.json", name);
    File::open(filename).unwrap()
}
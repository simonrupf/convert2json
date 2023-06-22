extern crate serde_json;
extern crate serde_yaml;
use std::io::{stdin, stdout};

fn main() {
    let value: serde_json::Value = serde_yaml::from_reader(stdin()).unwrap();
    serde_json::to_writer(stdout(), &value).unwrap();
}
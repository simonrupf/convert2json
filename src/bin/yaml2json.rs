extern crate serde_yaml;
use convert2json::lib::to_json;
use std::io::stdin;

fn main() {
    to_json(&serde_yaml::from_reader(stdin()));
}

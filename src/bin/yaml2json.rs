extern crate serde_yaml;
use std::io::stdin;
use convert2json::lib::to_json;

fn main() {
    to_json(serde_yaml::from_reader(stdin()));
}

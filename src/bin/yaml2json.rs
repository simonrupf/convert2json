extern crate serde_yaml;
extern crate yaml2json;
use std::io::stdin;
use yaml2json::lib::to_json;

fn main() {
    to_json(serde_yaml::from_reader(stdin()));
}
extern crate toml;
extern crate yaml2json;
use yaml2json::lib::{stdin_to_string, to_json};

fn main() {
    let buffer = stdin_to_string();
    to_json(toml::from_str(&buffer));
}
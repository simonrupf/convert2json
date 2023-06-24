extern crate toml;
use convert2json::lib::{stdin_to_string, to_json};

fn main() {
    let buffer = stdin_to_string();
    to_json(toml::from_str(&buffer));
}

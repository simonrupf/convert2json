extern crate serde_json;
extern crate serde_yaml;
use std::io::{stdin, stdout};
use std::process::exit;

fn main() {
    let value: serde_json::Value = match serde_yaml::from_reader(stdin()) {
        Ok(parsed_yaml) => parsed_yaml,
        Err(e) => {
            eprintln!("Error parsing input: {e:?}");
            exit(1);
        }
    };
    if let Err(e) = serde_json::to_writer(stdout(), &value) {
        eprintln!("Error serializing output: {e:?}");
        exit(2);
    }
}
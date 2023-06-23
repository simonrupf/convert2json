extern crate serde_json;
extern crate toml;
use std::io::{read_to_string, stdin, stdout};
use std::process::exit;

fn main() {
    let buffer = match read_to_string(stdin()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input: {0}", e.to_string());
            exit(1);
        }
    };
    let value: serde_json::Value = match toml::from_str(&buffer) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing input: {0}", e.to_string());
            exit(2);
        }
    };
    if let Err(e) = serde_json::to_writer(stdout(), &value) {
        eprintln!("Error serializing output: {0}", e.to_string());
        exit(3);
    }
}
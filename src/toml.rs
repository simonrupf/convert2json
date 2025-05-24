#![cfg(any(feature = "toml", feature = "toml2json", feature = "tq"))]
use super::exit;
use serde_json::Value;
use std::io::{read_to_string, Read};
use toml::from_str;

pub fn from_reader<R: Read>(reader: R) -> Value {
    match read_to_string(reader) {
        Ok(data) => match from_str(&data) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error parsing input: {e}");
                exit(super::Error::InputParsing as i32);
            }
        },
        Err(e) => {
            eprintln!("Error reading input: {e}");
            exit(super::Error::InputReading as i32);
        }
    }
}

#![cfg(any(feature = "toml", feature = "toml2json", feature = "tq"))]
use super::exit;
use serde_json::Value;
use std::io::{read_to_string, Read};
use toml::{de::Error, from_str};

pub fn from_reader<R: Read>(reader: R) -> Result<Value, Error> {
    match read_to_string(reader) {
        Ok(data) => from_str(&data),
        Err(e) => {
            eprintln!("Error reading input: {e}");
            exit(super::Error::InputReading as i32);
        }
    }
}

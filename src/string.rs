#![cfg(any(feature = "toml", feature = "toml2json", feature = "tq"))]
use super::{exit, Error};
use std::io::{read_to_string, Read};

pub fn from_reader<R: Read>(reader: R) -> String {
    match read_to_string(reader) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input: {e}");
            exit(Error::InputReading as i32);
        }
    }
}

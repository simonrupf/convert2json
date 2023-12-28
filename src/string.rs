#![cfg(any(feature = "toml", feature = "toml2json", feature = "tq"))]
use super::{exit, Error};
use std::fs::File;
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

pub fn from(file_name: &String) -> String {
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {file_name}: {e}");
            exit(Error::FileOpening as i32);
        }
    };
    match read_to_string(file) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file {file_name}: {e}");
            exit(Error::FileReading as i32);
        }
    }
}

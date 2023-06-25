use super::{exit, stdin_reader, Error};
use std::fs::File;
use std::io::read_to_string;

pub fn from_stdin() -> String {
    match read_to_string(stdin_reader()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input: {e}");
            exit(Error::InputReading as i32);
        }
    }
}

pub fn from(files: &Vec<String>) -> String {
    if !files.is_empty() {
        let file_name = &files[0];
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
    } else {
        from_stdin()
    }
}

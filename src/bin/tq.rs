extern crate toml;
use std::fs::File;
use std::io::read_to_string;
use std::process::exit;
use yaml2json::lib::{Error, parse_args, stdin_to_string, to_jq};

fn main() {
    let (arguments, files) = parse_args();

    let buffer: String;
    if files.len() > 0 {
        let file_name = &files[0];
        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {file_name}: {0}", e.to_string());
                exit(Error::FileOpening as i32);
            }
        };
        buffer = match read_to_string(file) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error reading file {file_name}: {0}", e.to_string());
                exit(Error::FileReading as i32);
            }
        };
    } else {
        buffer = stdin_to_string();
    }

    to_jq(toml::from_str(&buffer), arguments);
}
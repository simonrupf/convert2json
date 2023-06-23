extern crate serde_json;
extern crate serde_yaml;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::process::exit;
use yaml2json::lib::{Error, parse_args, to_jq};

fn main() {
    let (arguments, files) = parse_args();

    let reader: Box<dyn BufRead>;
    if files.len() > 0 {
        let file_name = &files[0];
        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {file_name}: {0}", e.to_string());
                exit(Error::FileOpening as i32);
            }
        };
        reader = Box::new(BufReader::new(file));
    } else {
        reader = Box::new(BufReader::new(stdin()));
    }

    to_jq(serde_yaml::from_reader(reader), arguments);
}
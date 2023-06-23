extern crate serde_json;
extern crate serde_yaml;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
use std::process::{exit, Command, Stdio};

fn main() {
    let mut arguments: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    let mut args_done = false;
    for arg in args().skip(1) {
        if args_done {
            files.push(arg);
            continue;
        } else if arg == "--" {
            args_done = true;
            continue;
        }
        arguments.push(arg);
        continue;
    }

    let reader: Box<dyn BufRead>;
    if files.len() > 0 {
        let file_name = &files[0];
        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {file_name}: {0}", e.to_string());
                exit(1);
            }
        };
        reader = Box::new(BufReader::new(file));
    } else {
        reader = Box::new(BufReader::new(stdin()));
    }

    let value: serde_json::Value = match serde_yaml::from_reader(reader) {
        Ok(parsed_yaml) => parsed_yaml,
        Err(e) => {
            eprintln!("Error parsing input: {0}", e.to_string());
            exit(2);
        }
    };
    let mut child = match Command::new("jq")
        .args(arguments)
        .stdin(Stdio::piped())
        .spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error calling jq: {0}", e.to_string());
            exit(3);
        }
    };
    let child_stdin = match child.stdin.as_mut() {
        Some(stdin) => stdin,
        None => {
            eprintln!("Error opening jq's STDIN for writing");
            exit(4);
        }
    };
    let mut exit_code = 0;
    if let Err(e) = serde_json::to_writer(child_stdin, &value) {
        eprintln!("Error serializing output: {0}", e.to_string());
        exit_code = 5;
    }
    if let Err(e) = child.wait() {
        eprintln!("Error waiting on jq: {0}", e.to_string());
        exit_code = 6;
    }
    exit(exit_code);
}
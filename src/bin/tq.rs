extern crate serde_json;
extern crate toml;
use std::env::args;
use std::fs::File;
use std::io::{read_to_string, stdin};
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

    let buffer: String;
    if files.len() > 0 {
        let file_name = &files[0];
        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {file_name}: {0}", e.to_string());
                exit(1);
            }
        };
        buffer = match read_to_string(file) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error reading file {file_name}: {0}", e.to_string());
                exit(2);
            }
        };
    } else {
        buffer = match read_to_string(stdin()) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error reading input: {0}", e.to_string());
                exit(3);
            }
        };
    }

    let value: serde_json::Value = match toml::from_str(&buffer) {
        Ok(parsed_yaml) => parsed_yaml,
        Err(e) => {
            eprintln!("Error parsing input: {0}", e.to_string());
            exit(4);
        }
    };
    let mut child = match Command::new("jq")
        .args(arguments)
        .stdin(Stdio::piped())
        .spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error calling jq: {0}", e.to_string());
            exit(5);
        }
    };
    let child_stdin = match child.stdin.as_mut() {
        Some(stdin) => stdin,
        None => {
            eprintln!("Error opening jq's STDIN for writing");
            exit(6);
        }
    };
    let mut exit_code = 0;
    if let Err(e) = serde_json::to_writer(child_stdin, &value) {
        eprintln!("Error serializing output: {0}", e.to_string());
        exit_code = 7;
    }
    if let Err(e) = child.wait() {
        eprintln!("Error waiting on jq: {0}", e.to_string());
        exit_code = 8;
    }
    exit(exit_code);
}
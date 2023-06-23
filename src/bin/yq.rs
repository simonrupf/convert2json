extern crate serde_json;
extern crate serde_yaml;
use std::env::args;
use std::io::stdin;
use std::process::{exit, Command, Stdio};

fn main() {
    let value: serde_json::Value = match serde_yaml::from_reader(stdin()) {
        Ok(parsed_yaml) => parsed_yaml,
        Err(e) => {
            eprintln!("Error parsing input: {0}", e.to_string());
            exit(1);
        }
    };
    let mut child = match Command::new("jq")
        .args(args().skip(1))
        .stdin(Stdio::piped())
        .spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error calling jq: {0}", e.to_string());
            exit(2);
        }
    };
    let child_stdin = match child.stdin.as_mut() {
        Some(stdin) => stdin,
        None => {
            eprintln!("Error opening jq's STDIN for writing");
            exit(3);
        }
    };
    let mut exit_code = 0;
    if let Err(e) = serde_json::to_writer(child_stdin, &value) {
        eprintln!("Error serializing output: {0}", e.to_string());
        exit_code = 4;
    }
    if let Err(e) = child.wait() {
        eprintln!("Error waiting on jq: {0}", e.to_string());
        exit_code = 5;
    }
    exit(exit_code);
}
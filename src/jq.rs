use super::{exit, stdin_reader, to_json_value, Error};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn parse_args() -> (Vec<String>, Vec<String>) {
    let mut arguments: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    let mut args_done = false;
    for arg in args().skip(1) {
        if args_done || Path::new(&arg).is_file() {
            files.push(arg);
        } else if arg == "--" {
            args_done = true;
        } else {
            arguments.push(arg);
        }
    }
    (arguments, files)
}

pub fn jq<E>(input: &Result<serde_json::Value, E>, arguments: &Vec<String>)
where
    E: ToString,
{
    let mut child = match Command::new("jq")
        .args(arguments)
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error calling jq: {e}");
            exit(Error::JqCalling as i32);
        }
    };
    let child_stdin = match child.stdin.as_mut() {
        Some(stdin) => stdin,
        None => {
            eprintln!("Error opening jq's STDIN for writing");
            exit(Error::JqPiping as i32);
        }
    };
    let mut exit_code: i32 = 0;
    if let Err(e) = serde_json::to_writer(child_stdin, to_json_value(input)) {
        eprintln!("Error serializing output: {e}");
        exit_code = Error::OutputSerialization as i32;
    }
    if let Err(e) = child.wait() {
        eprintln!("Error waiting on jq: {e}");
        exit_code = Error::JqWaiting as i32;
    }
    exit(exit_code);
}

pub fn reader(files: &Vec<String>) -> Box<dyn BufRead> {
    if !files.is_empty() {
        let file_name = &files[0];
        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {file_name}: {e}");
                exit(Error::FileOpening as i32);
            }
        };
        Box::new(BufReader::new(file))
    } else {
        Box::new(stdin_reader())
    }
}

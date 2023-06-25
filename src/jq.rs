use super::{exit, json::to_value, stdin_reader, Error};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub struct Jq {
    child: Child,
}

impl Jq {
    pub fn new(arguments: &Vec<String>) -> Self {
        let child = match Command::new("jq")
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
        Self { child }
    }

    pub fn write<E>(&mut self, input: &Result<serde_json::Value, E>)
    where
        E: ToString,
    {
        let child_stdin = match self.child.stdin.as_mut() {
            Some(stdin) => stdin,
            None => {
                eprintln!("Error opening jq's STDIN for writing");
                exit(Error::JqPiping as i32);
            }
        };
        if let Err(e) = serde_json::to_writer(child_stdin, to_value(input)) {
            eprintln!("Error serializing output: {e}");
            self.wait();
            exit(Error::OutputSerialization as i32);
        }
    }

    fn wait(&mut self) {
        if let Err(e) = self.child.wait() {
            eprintln!("Error waiting on jq: {e}");
            exit(Error::JqWaiting as i32);
        }
    }
}

impl Drop for Jq {
    fn drop(&mut self) {
        self.wait();
    }
}

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

pub fn readers(files: &Vec<String>) -> Vec<Box<dyn BufRead>> {
    if !files.is_empty() {
        let mut file_readers: Vec<Box<dyn BufRead>> = vec![];
        for file_name in files {
            let file = match File::open(file_name) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file {file_name}: {e}");
                    exit(Error::FileOpening as i32);
                }
            };
            file_readers.push(Box::new(BufReader::new(file)))
        }
        file_readers
    } else {
        vec![Box::new(stdin_reader())]
    }
}

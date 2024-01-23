#![cfg(any(feature = "cq", feature = "tq", feature = "xq", feature = "yq"))]
use super::{exit, stdin_reader, Error};
use serde::Serialize;
use std::collections::HashMap;
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

    pub fn write<T>(&mut self, input: &T)
    where
        T: ?Sized + Serialize,
    {
        let stdin = self.child.stdin.as_mut();
        if stdin.is_none() {
            eprintln!("Error opening jq's STDIN for writing");
            self.wait();
            exit(Error::JqPiping as i32);
        }
        if let Err(e) = &serde_json::to_writer(stdin.unwrap(), input) {
            eprintln!("Error serializing output: {e}");
            self.wait();
            exit(Error::OutputSerialization as i32);
        };
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
    #[derive(PartialEq)]
    enum ArgType {
        Csv,
        Jq,
    }
    let mut arguments: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    let mut args_done = false;
    let mut skip: u8 = 0;
    let skip_args: HashMap<&str, (u8, ArgType)> = HashMap::from([
        ("--no-trim",   (0, ArgType::Csv)),
        ("-d",          (1, ArgType::Csv)),
        ("--delimiter", (1, ArgType::Csv)),
        ("-q",          (1, ArgType::Csv)),
        ("--quote",     (1, ArgType::Csv)),
        ("-E",          (1, ArgType::Csv)),
        ("--escape",    (1, ArgType::Csv)),
        ("-f",          (1, ArgType::Jq)),
        ("--from-file", (1, ArgType::Jq)),
        ("--run-tests", (1, ArgType::Jq)),
        ("--slurpfile", (2, ArgType::Jq)),
        ("--rawfile",   (2, ArgType::Jq)),
    ]);
    let mut skip_and_push = false;
    for arg in args().skip(1) {
        if skip > 0 {
            skip -= 1;
            if !skip_and_push {
                continue;
            }
        } else if let Some((args_to_skip, arg_type)) = skip_args.get(&arg.as_str()) {
            skip = *args_to_skip;
            skip_and_push = *arg_type == ArgType::Jq;
            if !skip_and_push {
                continue;
            }
        } else if args_done || Path::new(&arg).is_file() {
            files.push(arg);
            continue;
        } else if arg == "--" {
            args_done = true;
            continue;
        }
        arguments.push(arg);
        if skip_and_push && skip == 0 {
            skip_and_push = false;
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

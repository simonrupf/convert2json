#![cfg(any(
    feature = "bsonq",
    feature = "cborq",
    feature = "cq",
    feature = "iq",
    feature = "msgq",
    feature = "plistq",
    feature = "tq",
    feature = "xq",
    feature = "yq"
))]
use super::{exit, stdin_reader, Error};
use serde::Serialize;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind::NotFound};
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub struct Jq {
    child: Child,
    program: String,
}

impl Jq {
    pub fn new(arguments: &Vec<String>) -> Self {
        let mut program = "jaq".to_string();
        let child = match Command::new(&program)
            .args(arguments)
            .stdin(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                if NotFound == e.kind() {
                    program = "jq".to_string();
                    match Command::new(&program)
                        .args(arguments)
                        .stdin(Stdio::piped())
                        .spawn()
                    {
                        Ok(child) => child,
                        Err(e) => {
                            eprintln!("Error calling {program}: {e}");
                            exit(Error::JqCalling as i32);
                        }
                    }
                } else {
                    eprintln!("Error calling {program}: {e}");
                    exit(Error::JaqCalling as i32);
                }
            }
        };
        Self { child, program }
    }

    fn is_jq(&mut self) -> bool {
        self.program == "jq"
    }

    pub fn write<T>(&mut self, input: &T)
    where
        T: ?Sized + Serialize,
    {
        let stdin = self.child.stdin.as_mut();
        if stdin.is_none() {
            eprintln!("Error opening {}'s STDIN for writing", self.program);
            self.wait();
            exit(match self.is_jq() {
                true => Error::JqPiping,
                false => Error::JaqPiping,
            } as i32);
        }
        if let Err(e) = &serde_json::to_writer(stdin.unwrap(), input) {
            eprintln!("Error serializing output: {e}");
            self.wait();
            exit(Error::OutputSerialization as i32);
        };
    }

    fn wait(&mut self) {
        if let Err(e) = self.child.wait() {
            eprintln!("Error waiting on {}: {e}", self.program);
            exit(match self.is_jq() {
                true => Error::JqWaiting,
                false => Error::JaqWaiting,
            } as i32);
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
    let skip_args = [
        ("--no-trim", 0, ArgType::Csv),
        ("-d", 1, ArgType::Csv),
        ("--delimiter", 1, ArgType::Csv),
        ("-q", 1, ArgType::Csv),
        ("--quote", 1, ArgType::Csv),
        ("-E", 1, ArgType::Csv),
        ("--escape", 1, ArgType::Csv),
        ("-f", 1, ArgType::Jq),
        ("--from-file", 1, ArgType::Jq),
        ("--run-tests", 1, ArgType::Jq),
        ("--slurpfile", 2, ArgType::Jq),
        ("--rawfile", 2, ArgType::Jq),
    ];
    let mut skip_and_push = false;
    for arg in args().skip(1) {
        if skip > 0 {
            skip -= 1;
            if !skip_and_push {
                continue;
            }
        } else if let Some((_, args_to_skip, arg_type)) =
            skip_args.iter().find(|&item| item.0 == arg.as_str())
        {
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

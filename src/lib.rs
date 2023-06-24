pub mod lib {
    extern crate serde_json;
    use std::env::args;
    use std::fs::File;
    use std::io::{read_to_string, stdin, stdout, BufRead, BufReader};
    use std::process::{exit, Command, Stdio};

    pub enum Error {
        InputParsing = 1,
        InputReading,
        FileOpening,
        FileReading,
        JqCalling,
        JqPiping,
        JqWaiting,
        OutputSerialization,
    }

    pub fn parse_args() -> (Vec<String>, Vec<String>) {
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
        (arguments, files)
    }

    pub fn reader_from(files: Vec<String>) -> Box<dyn BufRead> {
        if files.len() > 0 {
            let file_name = &files[0];
            let file = match File::open(file_name) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file {file_name}: {0}", e.to_string());
                    exit(Error::FileOpening as i32);
                }
            };
            Box::new(BufReader::new(file))
        } else {
            Box::new(BufReader::new(stdin()))
        }
    }

    pub fn string_from(files: Vec<String>) -> String {
        if files.len() > 0 {
            let file_name = &files[0];
            let file = match File::open(file_name) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file {file_name}: {0}", e.to_string());
                    exit(Error::FileOpening as i32);
                }
            };
            match read_to_string(file) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error reading file {file_name}: {0}", e.to_string());
                    exit(Error::FileReading as i32);
                }
            }
        } else {
            stdin_to_string()
        }
    }

    pub fn stdin_to_string() -> String {
        match read_to_string(stdin()) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error reading input: {0}", e.to_string());
                exit(Error::InputReading as i32);
            }
        }
    }

    pub fn to_json<E>(input: Result<serde_json::Value, E>)
    where
        E: ToString,
    {
        let value: serde_json::Value = match input {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error parsing input: {0}", e.to_string());
                exit(Error::InputParsing as i32);
            }
        };
        if let Err(e) = serde_json::to_writer(stdout(), &value) {
            eprintln!("Error serializing output: {0}", e.to_string());
            exit(Error::OutputSerialization as i32);
        }
    }

    pub fn to_jq<E>(input: Result<serde_json::Value, E>, arguments: Vec<String>)
    where
        E: ToString,
    {
        let value: serde_json::Value = match input {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error parsing input: {0}", e.to_string());
                exit(Error::InputParsing as i32);
            }
        };
        let mut child = match Command::new("jq")
            .args(arguments)
            .stdin(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                eprintln!("Error calling jq: {0}", e.to_string());
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
        if let Err(e) = serde_json::to_writer(child_stdin, &value) {
            eprintln!("Error serializing output: {0}", e.to_string());
            exit_code = Error::OutputSerialization as i32;
        }
        if let Err(e) = child.wait() {
            eprintln!("Error waiting on jq: {0}", e.to_string());
            exit_code = Error::JqWaiting as i32;
        }
        exit(exit_code);
    }
}

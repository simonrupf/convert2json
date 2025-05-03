#![cfg(any(
    feature = "bson2json",
    feature = "cbor2json",
    feature = "csv2json",
    feature = "ini2json",
    feature = "msg2json",
    feature = "plist2json",
    feature = "toml2json",
    feature = "xml2json",
    feature = "yaml2json"
))]
use super::{exit, stdin_reader, Error, HELP_ARGS};
use serde::Serialize;
use std::env::args;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader};
use std::path::Path;

pub fn parse_args() -> impl Iterator<Item = Box<dyn BufRead>> {
    let mut file_readers: Vec<Box<dyn BufRead>> = vec![];
    let mut arguments = args();
    if arguments.len() > 1 {
        let tool_path = arguments.next().unwrap();
        for arg in arguments {
            if HELP_ARGS.contains(&arg.as_str()) {
                let suffix = "2json";
                let tool = Path::new(&tool_path)
                    .file_name()
                    .unwrap_or_else(|| OsStr::new(suffix))
                    .to_string_lossy();
                let format = tool[..tool.len() - suffix.len()].to_uppercase();
                eprintln!(
                    "Usage: {tool} [-h|--help] [{format} files... | < <{format} input>]\n\nReads {format} from files or standard input and converts this to JSON, emitted on standard output. Any errors are reported to standard error and result in a non-zero exit code."
                );
                exit(0);
            }
            if Path::new(&arg).is_file() {
                let file = match File::open(&arg) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Error opening file {arg}: {e}");
                        exit(Error::FileOpening as i32);
                    }
                };
                file_readers.push(Box::new(BufReader::new(file)))
            }
        }
    }
    if file_readers.is_empty() {
        file_readers.push(Box::new(stdin_reader()));
    }
    file_readers.into_iter()
}

pub fn stdout_writer<T>(input: T)
where
    T: Sized + Serialize,
{
    if let Err(e) = serde_json::to_writer(stdout(), &input) {
        eprintln!("Error serializing output: {e}");
        exit(Error::OutputSerialization as i32);
    }
}

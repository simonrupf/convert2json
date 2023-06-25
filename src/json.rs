use super::{exit, to_json_value, Error};
use std::env::args;
use std::ffi::OsStr;
use std::io::stdout;
use std::path::Path;

pub fn parse_args() {
    let mut arguments = args();
    if arguments.len() < 2 {
        return;
    }
    let tool_path = arguments.nth(0).unwrap();
    for arg in arguments {
        match arg.as_str() {
            "-h" | "-?" | "--help" | "-help" => {
                let suffix = "2json";
                let tool = Path::new(&tool_path)
                    .file_name()
                    .unwrap_or(OsStr::new(suffix))
                    .to_string_lossy();
                let format = tool[..tool.len() - suffix.len()].to_uppercase();
                eprintln!(
                    "Usage: {tool} [-h|--help] < <{format} input>\n\nReads {format} from standard input and converts it to JSON, emitted on standard output. Any errors are reported to standard error and result in a non-zero exit code."
                );
                exit(0);
            }
            "--" => break,
            _ => continue,
        };
    }
}

pub fn stdout_writer<E>(input: &Result<serde_json::Value, E>)
where
    E: ToString,
{
    if let Err(e) = serde_json::to_writer(stdout(), to_json_value(input)) {
        eprintln!("Error serializing output: {e}");
        exit(Error::OutputSerialization as i32);
    }
}

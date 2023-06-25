use super::{exit, to_json_value, Error};
use std::env::args;
use std::io::stdout;

pub fn parse_args() {
    let mut arguments = args();
    if arguments.len() < 2 {
        return;
    }
    let tool_path = arguments.nth(0).unwrap();
    for arg in arguments {
        match arg.as_str() {
            "-h" | "-?" | "--help" | "-help" => {
                let start = match tool_path.rfind('/') {
                    Some(offset) => offset + 1,
                    None => 0,
                };
                let tool = &tool_path[start..];
                let end = tool.len() - "2json".len();
                let format = tool[..end].to_uppercase();
                eprintln!(
                    "Usage: {tool} [-h|--help] << <{format} input>\n\nReads {format} from standard input, converts it to JSON and emits that to standard output. Any errors are reported to standard error and result in a non-zero exit code."
                );
                exit(0);
            },
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

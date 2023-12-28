pub mod csv;
pub mod jq;
pub mod json;
pub mod string;
pub mod yaml;

extern crate serde_json;
use std::io::{stdin, IsTerminal, StdinLock};
use std::process::exit;

// Error exit codes, starting at 1
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

pub fn stdin_reader() -> StdinLock<'static> {
    let stdin = stdin();
    if stdin.is_terminal() {
        eprintln!("Error reading input: Did you forget to pipe something into us?");
        exit(Error::InputReading as i32);
    }
    stdin.lock()
}

pub fn to_value<E>(input: &Result<serde_json::Value, E>) -> &serde_json::Value
where
    E: ToString,
{
    match input {
        Ok(data) => data,
        Err(e) => {
            // these give more detailed information using to_string() over std::fmt::display
            eprintln!("Error parsing input: {}", e.to_string());
            exit(Error::InputParsing as i32);
        }
    }
}

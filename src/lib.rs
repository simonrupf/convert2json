#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
pub mod csv;
pub mod jq;
pub mod json;
pub mod seek;
pub mod toml;
pub mod xml;
pub mod yaml;

extern crate serde_json;
use std::io::{stdin, IsTerminal, StdinLock};
use std::process::exit;

const HELP_ARGS: [&str; 4] = ["-h", "-?", "--help", "-help"];

// Error exit codes, starting at 1
#[repr(u8)]
pub enum Error {
    InputParsing = 1,
    InputReading,
    FileOpening,
    FileReading,
    JqCalling,
    JqPiping,
    JqWaiting,
    OutputSerialization,
    ArgumentParsing,
    JaqCalling,
    JaqPiping,
    JaqWaiting,
}

pub fn stdin_reader() -> StdinLock<'static> {
    let stdin = stdin();
    if stdin.is_terminal() {
        eprintln!("Error reading input: Did you forget to pipe something into us?");
        exit(Error::InputReading as i32);
    }
    stdin.lock()
}

pub fn to_value<E>(input: Result<serde_json::Value, E>) -> serde_json::Value
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

#![cfg(any(feature = "csv", feature = "csv2json", feature = "cq"))]
extern crate csv;

use super::{exit, Error};
use csv::{ReaderBuilder, Trim};
use pico_args::{Arguments, Error as picoError};
use std::collections::HashMap;
use std::io::Read;

const HELP: &str = "\
Usage: csv2json [-d <delimiter>] [-q <quote>] [-E <escape>] [--no-trim] [files...]

Reads CSV from files or standard input and converts this to JSON, emitted on standard output. Any errors are reported to standard error and result in a non-zero exit code.

Options:
  -d, --delimiter   field delimiter to use when parsing CSV, defaults to: ,
                    (comma)
  -q, --quote       quote character to use when parsing CSV, defaults to: \"
                    (double quote)
  -E, --escape      escape character to use when parsing CSV, to escape quote
                    characters within a field. By default, quotes get escaped by
                    doubling them.
  --no-trim         do not trim headers & fields. By default, both get trimmed
                    of starting or trailing whitespace characters.
  -h, --help        display usage information
";

pub type CsvMap = HashMap<String, serde_json::Value>;

struct CsvParameters {
    delimiter: Option<u8>,
    quote: Option<u8>,
    escape: Option<u8>,
    no_trim: bool,
}

pub struct CsvReader {
    read: ReaderBuilder,
}

impl CsvReader {
    pub fn new(usage: bool) -> Self {
        let arguments = match Self::args(usage) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error {e}");
                exit(Error::ArgumentParsing as i32);
            }
        };
        let mut read = ReaderBuilder::new();
        read.flexible(true);
        if let Some(delimiter) = arguments.delimiter {
            read.delimiter(delimiter);
        }
        if let Some(quote) = arguments.quote {
            read.quote(quote);
        }
        if arguments.escape.is_some() {
            // note that setting this to None would disable escape sequences entirely
            read.escape(arguments.escape).double_quote(false);
        }
        if !arguments.no_trim {
            read.trim(Trim::All);
        }
        Self { read }
    }

    pub fn append<R: Read>(&mut self, results: &mut Vec<CsvMap>, reader: R) {
        for row in self.read.from_reader(reader).deserialize() {
            let record: CsvMap = match row {
                Ok(values) => values,
                Err(e) => {
                    eprintln!("Error parsing input: {e}");
                    exit(Error::InputParsing as i32);
                }
            };
            results.push(record);
        }
    }

    fn args(usage: bool) -> Result<CsvParameters, picoError> {
        let mut pargs = Arguments::from_env();
        if usage && pargs.contains(["-h", "--help"]) {
            eprintln!("{HELP}");
            exit(0);
        }
        let args = CsvParameters {
            delimiter: pargs.opt_value_from_fn(["-d", "--delimiter"], Self::arg_u8)?,
            quote: pargs.opt_value_from_fn(["-q", "--quote"], Self::arg_u8)?,
            escape: pargs.opt_value_from_fn(["-E", "--escape"], Self::arg_u8)?,
            no_trim: pargs.contains("--no-trim"),
        };
        Ok(args)
    }

    fn arg_u8(s: &str) -> Result<u8, &'static str> {
        if s.len() != 1 {
            return Err("argument requires a single character");
        }
        match s.chars().next() {
            Some(c) => Ok(c as u8),
            None => Err("argument is missing a character"),
        }
    }
}

#![cfg(any(feature = "csv", feature = "csv2json", feature = "cq"))]
extern crate csv;

use super::{exit, Error};
use argh::FromArgs;
use csv::{ReaderBuilder, Trim};
use std::collections::HashMap;
use std::env::args;
use std::io::Read;

pub type CsvMap = HashMap<String, serde_json::Value>;

#[derive(FromArgs)]
/// Reads CSV from files or standard input and converts this to JSON, emitted on
/// standard output. Any errors are reported to standard error and result in a
/// non-zero exit code.
struct CsvParameters {
    /// field delimiter to use when parsing CSV, defaults to: , (comma)
    #[argh(option, short = 'd')]
    delimiter: Option<char>,

    /// quote character to use when parsing CSV, defaults to: " (double quote)
    #[argh(option, short = 'q')]
    quote: Option<char>,

    /// escape character to use when parsing CSV, to escape quote characters
    /// within a field. By default, quotes get escaped by doubling them.
    #[argh(option, short = 'E')]
    escape: Option<char>,

    /// do not trim headers & fields. By default, both get trimmed of starting
    /// or trailing whitespace characters.
    #[argh(switch)]
    no_trim: bool,

    /// one or more CSV files to read
    #[allow(dead_code)]
    #[argh(positional)]
    files: Vec<String>,
}

pub struct CsvReader {
    read: ReaderBuilder,
}

impl CsvReader {
    pub fn new() -> Self {
        let arguments: CsvParameters = argh::from_env();
        let mut read = ReaderBuilder::new();
        read.flexible(true);
        if let Some(delimiter) = arguments.delimiter {
            read.delimiter(delimiter as u8);
        }
        if let Some(quote) = arguments.quote {
            read.quote(quote as u8);
        }
        if let Some(escape) = arguments.escape {
            // note that setting this to None would disable escape sequences entirely
            read.escape(Some(escape as u8)).double_quote(false);
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
}

impl Default for CsvReader {
    fn default() -> Self {
        let mut read = ReaderBuilder::new();
        let mut do_trim = true;
        let mut read_var: i8 = -1;
        let csv_args = ["-d", "-q", "-E"];
        read.flexible(true);
        for arg in args().skip(1) {
            if arg == "--no-trim" {
                do_trim = false;
            } else if read_var > -1 && read_var < 3 && arg.len() == 1 {
                match read_var {
                    0 => read.delimiter(arg.as_str().chars().next().unwrap() as u8),
                    1 => read.quote(arg.as_str().chars().next().unwrap() as u8),
                    2 => read
                        .escape(Some(arg.as_str().chars().next().unwrap() as u8))
                        .double_quote(false),
                    _ => &mut read,
                };
                read_var = -1;
            } else if csv_args.contains(&arg.as_str()) {
                read_var = match csv_args.iter().position(|&flag| flag == arg) {
                    Some(index) => index as i8,
                    None => -1,
                }
            } else {
                read_var = -1;
            }
        }
        if do_trim {
            read.trim(Trim::All);
        }
        Self { read }
    }
}

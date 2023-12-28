#![cfg(any(feature = "csv", feature = "csv2json", feature = "cq"))]
extern crate csv;

use super::{exit, Error};
use argh::FromArgs;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

#[derive(Deserialize, Serialize)]
pub struct CsvMap {
    #[serde(flatten)]
    values: HashMap<String, String>,
}

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
        read.flexible(true);
        Self { read }
    }
}

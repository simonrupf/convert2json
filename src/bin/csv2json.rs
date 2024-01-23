#![cfg(feature = "csv2json")]
use convert2json::csv::{CsvMap, CsvReader};
use convert2json::json::{parse_args, stdout_writer};

fn main() {
    let mut csv = CsvReader::new(true);
    let mut results: Vec<CsvMap> = vec![];
    for reader in parse_args() {
        csv.append(&mut results, reader);
    }
    stdout_writer(&results);
}

#![cfg(feature = "csv2json")]
use convert2json::csv::CsvReader;
use convert2json::json::{parse_args, stdout_writer};

fn main() {
    let mut csv = CsvReader::new(true);
    parse_args().for_each(|reader| csv.append(reader));
    stdout_writer(&csv.results);
}

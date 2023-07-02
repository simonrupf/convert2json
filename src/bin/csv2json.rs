use convert2json::csv::{CsvMap, CsvReader};
use convert2json::json::{parse_args, stdout_writer};

#[cfg(feature = "csv2json")]
fn main() {
    let mut csv = CsvReader::new();
    let mut results: Vec<CsvMap> = vec![];
    for reader in parse_args() {
        csv.append(&mut results, reader);
    }
    stdout_writer(&results);
}

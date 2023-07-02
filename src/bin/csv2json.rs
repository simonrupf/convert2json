use convert2json::csv::{append, CsvMap};
use convert2json::json::{parse_args, stdout_writer};

#[cfg(feature = "csv2json")]
fn main() {
    let mut results: Vec<CsvMap> = vec![];
    for reader in parse_args() {
        append(&mut results, reader);
    }
    stdout_writer(&results);
}

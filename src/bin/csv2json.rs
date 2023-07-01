use convert2json::csv::{append, CsvMap};
use convert2json::json::{parse_args, stdout_writer};
use convert2json::stdin_reader;

#[cfg(feature = "csv2json")]
fn main() {
    parse_args();
    let mut results: Vec<CsvMap> = vec![];
    append(&mut results, stdin_reader());
    stdout_writer(&results);
}

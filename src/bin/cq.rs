#![cfg(feature = "cq")]
use convert2json::csv::{CsvMap, CsvReader};
use convert2json::jq::{parse_args, readers, Jq};

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    let mut csv: CsvReader = CsvReader::new(false);
    let mut results: Vec<CsvMap> = vec![];
    for reader in readers(&files) {
        csv.append(&mut results, reader);
    }
    jq.write(&results);
}

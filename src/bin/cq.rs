#![cfg(feature = "cq")]
use convert2json::csv::CsvReader;
use convert2json::jq::{parse_args, readers, Jq};

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    let mut csv: CsvReader = CsvReader::new(false);
    for reader in readers(&files) {
        csv.append(reader);
    }
    jq.write(&csv.results);
}

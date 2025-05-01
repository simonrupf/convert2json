#![cfg(feature = "cq")]
use convert2json::csv::CsvReader;
use convert2json::jq::Jq;

fn main() {
    let mut jq = Jq::default();
    let mut csv: CsvReader = CsvReader::new(false);
    for reader in jq.readers() {
        csv.append(reader);
    }
    jq.write(&csv.results);
}

#![cfg(feature = "cq")]
use convert2json::csv::CsvReader;
use convert2json::jq::Jq;

fn main() {
    let mut jq = Jq::default();
    let mut csv: CsvReader = CsvReader::new(false);
    jq.readers().for_each(|reader| csv.append(reader));
    jq.write(csv.results);
}

use convert2json::csv::{append, CsvMap};
use convert2json::jq::{parse_args, readers, Jq};

#[cfg(feature = "cq")]
fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    let mut results: Vec<CsvMap> = vec![];
    for reader in readers(&files) {
        append(&mut results, reader);
    }
    jq.write(&results);
}

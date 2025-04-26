#![cfg(feature = "rsv2json")]
use convert2json::json::{parse_args, stdout_writer};
use rsv_core::reader::Reader;
type VecOptStr = Vec<Option<String>>;

fn main() {
    let mut results: Vec<VecOptStr> = vec![];
    for reader in parse_args() {
        for result in Reader::from_reader(reader)
            .deserialize::<VecOptStr>()
            .flatten()
        {
            results.push(result);
        }
    }
    stdout_writer(&results);
}

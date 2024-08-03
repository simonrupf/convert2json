#![cfg(feature = "rsv2json")]
extern crate rsv_core;
use convert2json::json::{parse_args, stdout_writer};
type VecOptStr = Vec<Option<String>>;

fn main() {
    let mut results: Vec<VecOptStr> = vec![];
    for reader in parse_args() {
        for result in rsv_core::reader::Reader::from_reader(reader).deserialize::<VecOptStr>() {
            results.push(result.unwrap());
        }
    }
    stdout_writer(&results);
}

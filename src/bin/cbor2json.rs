#![cfg(feature = "cbor2json")]
extern crate ciborium;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&ciborium::from_reader(reader)));
    }
}

#![cfg(feature = "bson2json")]
use bson::from_reader;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&from_reader(reader)));
    }
}

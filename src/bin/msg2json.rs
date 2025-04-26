#![cfg(feature = "msg2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;
use rmp_serde::from_read;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&from_read(reader)));
    }
}

#![cfg(feature = "ini2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;
use serde_ini::de::from_read;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&from_read(reader)));
    }
}

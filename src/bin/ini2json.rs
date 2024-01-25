#![cfg(feature = "ini2json")]
extern crate serde_ini;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&serde_ini::de::from_read(reader)));
    }
}

#![cfg(feature = "toml2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::{string::from_reader, to_value};
use toml::from_str;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&from_str(&from_reader(reader))));
    }
}

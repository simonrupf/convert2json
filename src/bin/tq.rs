#![cfg(feature = "tq")]
use convert2json::jq::Jq;
use convert2json::{string::from_reader, to_value};
use toml::from_str;

fn main() {
    let mut jq = Jq::default();
    for reader in jq.readers() {
        jq.write(to_value(&from_str(&from_reader(reader))));
    }
}

#![cfg(feature = "iq")]
use convert2json::jq::Jq;
use convert2json::to_value;
use serde_ini::de::from_read;

fn main() {
    let mut jq = Jq::default();
    for reader in jq.readers() {
        jq.write(to_value(&from_read(reader)));
    }
}

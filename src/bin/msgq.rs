#![cfg(feature = "msgq")]
use convert2json::jq::Jq;
use convert2json::to_value;
use rmp_serde::from_read;

fn main() {
    let mut jq = Jq::default();
    for reader in jq.readers() {
        jq.write(to_value(&from_read(reader)));
    }
}

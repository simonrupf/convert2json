#![cfg(feature = "bsonq")]
use bson::from_reader;
use convert2json::jq::Jq;
use convert2json::to_value;

fn main() {
    let mut jq = Jq::default();
    for reader in jq.readers() {
        jq.write(to_value(&from_reader(reader)));
    }
}

#![cfg(feature = "cborq")]
extern crate ciborium;
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::to_value;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(to_value(&ciborium::from_reader(reader)));
    }
}

#![cfg(feature = "cborq")]
use ciborium::from_reader;
use convert2json::jq::Jq;
use convert2json::to_value;

fn main() {
    let mut jq = Jq::default();
    jq.readers()
        .map(from_reader)
        .map(to_value)
        .for_each(|value| jq.write(value));
}

#![cfg(feature = "tq")]
use convert2json::jq::Jq;
use convert2json::toml::from_reader;

fn main() {
    let mut jq = Jq::default();
    jq.readers()
        .map(from_reader)
        .for_each(|value| jq.write(value));
}

#![cfg(feature = "tq")]
use convert2json::jq::Jq;
use convert2json::{string::from_reader, to_value};
use toml::from_str;

fn main() {
    let mut jq = Jq::default();
    jq.readers()
        .map(|reader| from_str(&from_reader(reader)))
        .map(to_value)
        .for_each(|value| jq.write(value));
}

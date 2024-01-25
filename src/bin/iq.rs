#![cfg(feature = "iq")]
extern crate serde_ini;
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::to_value;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(to_value(&serde_ini::de::from_read(reader)));
    }
}

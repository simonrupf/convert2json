#![cfg(feature = "xq")]
extern crate serde_xml_rs;
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::to_value;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(to_value(&serde_xml_rs::from_reader(reader)));
    }
}

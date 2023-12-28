#![cfg(feature = "xml2json")]
extern crate serde_xml_rs;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&serde_xml_rs::from_reader(reader)));
    }
}

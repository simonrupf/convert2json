#![cfg(feature = "xml2json")]
extern crate quick_xml;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&quick_xml::de::from_reader(reader)));
    }
}

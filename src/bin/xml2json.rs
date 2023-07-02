extern crate serde_xml_rs;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;

#[cfg(feature = "xml2json")]
fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&serde_xml_rs::from_reader(reader)));
    }
}

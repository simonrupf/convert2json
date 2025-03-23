#![cfg(feature = "xml2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::xml::wrap_xml_reader;

fn main() {
    for reader in parse_args() {
        stdout_writer(&wrap_xml_reader(reader));
    }
}

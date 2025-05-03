#![cfg(feature = "xml2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::xml::wrap_xml_reader;

fn main() {
    parse_args().map(wrap_xml_reader).for_each(stdout_writer);
}

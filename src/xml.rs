#![cfg(any(feature = "xml", feature = "xml2json", feature = "xq"))]
use quick_xml::Reader;
use serde_json::Value;
use std::io::BufRead;
use xmltojson::read;

pub fn wrap_xml_reader<R: BufRead>(reader: R) -> Value {
    let mut xml_reader = Reader::from_reader(reader);
    let config = xml_reader.config_mut();
    config.expand_empty_elements = true;
    config.trim_text(true);
    read(&mut xml_reader, 0)
}

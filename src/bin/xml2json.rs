extern crate serde_xml_rs;
use std::io::stdin;
use convert2json::lib::to_json;

fn main() {
    to_json(serde_xml_rs::from_reader(stdin()));
}

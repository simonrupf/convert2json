extern crate serde_xml_rs;
use convert2json::lib::to_json;
use std::io::stdin;

fn main() {
    to_json(&serde_xml_rs::from_reader(stdin()));
}

extern crate serde_xml_rs;
extern crate yaml2json;
use std::io::stdin;
use yaml2json::lib::to_json;

fn main() {
    to_json(serde_xml_rs::from_reader(stdin()));
}
extern crate serde_xml_rs;
use convert2json::lib::{parse_args, reader_from, to_jq};

fn main() {
    let (arguments, files) = parse_args();
    to_jq(serde_xml_rs::from_reader(reader_from(files)), arguments);
}

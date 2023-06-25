extern crate serde_xml_rs;
use convert2json::jq::{jq, parse_args, reader};

#[cfg(feature = "xq")]
fn main() {
    let (arguments, files) = parse_args();
    jq(&serde_xml_rs::from_reader(reader(&files)), &arguments);
}

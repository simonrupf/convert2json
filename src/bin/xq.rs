#![cfg(feature = "xq")]
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::xml::wrap_xml_reader;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(&wrap_xml_reader(reader));
    }
}

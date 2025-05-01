#![cfg(feature = "xq")]
use convert2json::jq::Jq;
use convert2json::xml::wrap_xml_reader;

fn main() {
    let mut jq = Jq::default();
    for reader in jq.readers() {
        jq.write(&wrap_xml_reader(reader));
    }
}

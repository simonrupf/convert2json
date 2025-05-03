#![cfg(feature = "xq")]
use convert2json::jq::Jq;
use convert2json::xml::wrap_xml_reader;

fn main() {
    let mut jq = Jq::default();
    jq.readers()
        .map(wrap_xml_reader)
        .for_each(|value| jq.write(value));
}

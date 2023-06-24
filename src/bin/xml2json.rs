extern crate serde_xml_rs;
use convert2json::{stdin_reader, to_json};

#[cfg(feature = "xml2json")]
fn main() {
    to_json(&serde_xml_rs::from_reader(stdin_reader()));
}

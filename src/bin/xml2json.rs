extern crate serde_xml_rs;
use convert2json::{stdin_reader, stdout_writer};

#[cfg(feature = "xml2json")]
fn main() {
    stdout_writer(&serde_xml_rs::from_reader(stdin_reader()));
}

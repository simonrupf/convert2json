extern crate serde_xml_rs;
use convert2json::{json, stdin_reader};

#[cfg(feature = "xml2json")]
fn main() {
    json::parse_args();
    json::stdout_writer(&serde_xml_rs::from_reader(stdin_reader()));
}

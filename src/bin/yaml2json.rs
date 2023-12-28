#![cfg(feature = "yaml2json")]
extern crate serde_yaml;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::yaml::document_iterator;

fn main() {
    document_iterator(parse_args(), stdout_writer);
}

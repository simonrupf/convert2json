#![cfg(feature = "yaml2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::yaml::{document_iterator, options};

fn main() {
    let options = options(true);
    document_iterator(parse_args(), &options, stdout_writer);
}

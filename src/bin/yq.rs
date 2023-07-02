extern crate serde_yaml;
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::yaml::document_iterator;
use std::cell::RefCell;

#[cfg(feature = "yq")]
fn main() {
    let (arguments, files) = parse_args();
    let jq = RefCell::new(Jq::new(&arguments));
    document_iterator(readers(&files), |doc| jq.borrow_mut().write(doc));
}

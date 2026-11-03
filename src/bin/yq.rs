#![cfg(feature = "yq")]
use convert2json::jq::Jq;
use convert2json::yaml::{document_iterator, options};
use std::cell::RefCell;

fn main() {
    let options = options(false);
    let jq_ref = RefCell::new(Jq::default());
    let mut jq = jq_ref.borrow_mut();
    document_iterator(jq.readers(), &options, |doc| jq.write(doc));
}

#![cfg(feature = "yq")]
use convert2json::jq::Jq;
use convert2json::yaml::document_iterator;
use std::cell::RefCell;

fn main() {
    let jq_ref = RefCell::new(Jq::default());
    let mut jq = jq_ref.borrow_mut();
    document_iterator(jq.readers(), |doc| jq.write(doc));
}

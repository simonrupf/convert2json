#![cfg(feature = "rq")]
use convert2json::jq::{parse_args, readers, Jq};
use rsv_core::reader::Reader;
type VecOptStr = Vec<Option<String>>;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    let mut results: Vec<VecOptStr> = vec![];
    for reader in readers(&files) {
        for result in Reader::from_reader(reader)
            .deserialize::<VecOptStr>()
            .flatten()
        {
            results.push(result);
        }
    }
    jq.write(&results);
}

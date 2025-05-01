#![cfg(feature = "rq")]
use convert2json::jq::Jq;
use rsv_core::reader::Reader;
type VecOptStr = Vec<Option<String>>;

fn main() {
    let mut jq = Jq::default();
    let mut results: Vec<VecOptStr> = vec![];
    for reader in jq.readers() {
        for result in Reader::from_reader(reader)
            .deserialize::<VecOptStr>()
            .flatten()
        {
            results.push(result);
        }
    }
    jq.write(&results);
}

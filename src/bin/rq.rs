#![cfg(feature = "rq")]
use convert2json::jq::Jq;
use rsv_core::reader::Reader;
type VecOptStr = Vec<Option<String>>;

fn main() {
    let mut jq = Jq::default();
    let results = jq
        .readers()
        .flat_map(|reader| {
            Reader::from_reader(reader)
                .deserialize::<VecOptStr>()
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    jq.write(&results);
}

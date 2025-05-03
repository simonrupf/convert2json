#![cfg(feature = "rsv2json")]
use convert2json::json::{parse_args, stdout_writer};
use rsv_core::reader::Reader;
type VecOptStr = Vec<Option<String>>;

fn main() {
    let results = parse_args()
        .flat_map(|reader| {
            Reader::from_reader(reader)
                .deserialize::<VecOptStr>()
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    stdout_writer(results);
}

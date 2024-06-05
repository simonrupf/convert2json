#![cfg(feature = "tq")]
extern crate toml;
use convert2json::jq::{parse_args, Jq};
use convert2json::stdin_reader;
type VecOptStr = Vec<Option<String>>;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    let mut results: Vec<VecOptStr> = vec![];
    if files.is_empty() {
        for result in
            rsv_core::reader::Reader::from_reader(stdin_reader()).deserialize::<VecOptStr>()
        {
            results.push(result.unwrap());
        }
    } else {
        for file in files {
            for result in rsv_core::reader::Reader::from_path(&file)
                .unwrap()
                .deserialize::<VecOptStr>()
            {
                results.push(result.unwrap());
            }
        }
    }
    jq.write(&results);
}

#![cfg(feature = "plistq")]
use convert2json::jq::Jq;
use convert2json::seek::BufSeek;
use convert2json::to_value;
use plist::from_reader;

fn main() {
    let mut jq = Jq::default();
    jq.readers()
        .map(|reader| from_reader(BufSeek::new(reader)))
        .map(to_value)
        .for_each(|value| jq.write(value));
}

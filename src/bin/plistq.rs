#![cfg(feature = "plistq")]
use convert2json::jq::Jq;
use convert2json::seek::BufSeek;
use convert2json::to_value;
use plist::from_reader;

fn main() {
    let mut jq = Jq::default();
    for reader in jq.readers() {
        jq.write(to_value(&from_reader(BufSeek::new(reader))));
    }
}

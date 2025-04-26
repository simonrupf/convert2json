#![cfg(feature = "plistq")]
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::seek::BufSeek;
use convert2json::to_value;
use plist::from_reader;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(to_value(&from_reader(BufSeek::new(reader))));
    }
}

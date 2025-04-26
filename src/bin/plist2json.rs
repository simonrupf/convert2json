#![cfg(feature = "plist2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::seek::BufSeek;
use convert2json::to_value;
use plist::from_reader;

fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&from_reader(BufSeek::new(reader))));
    }
}

#![cfg(feature = "plist2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::seek::BufSeek;
use convert2json::to_value;
use plist::from_reader;

fn main() {
    parse_args()
        .map(|reader| from_reader(BufSeek::new(reader)))
        .map(to_value)
        .for_each(stdout_writer);
}

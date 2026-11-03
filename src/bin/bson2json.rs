#![cfg(feature = "bson2json")]
use bson::deserialize_from_reader;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;

fn main() {
    parse_args()
        .map(deserialize_from_reader)
        .map(to_value)
        .for_each(stdout_writer);
}

#![cfg(feature = "msg2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;
use rmp_serde::from_read;

fn main() {
    parse_args()
        .map(from_read)
        .map(to_value)
        .for_each(stdout_writer);
}

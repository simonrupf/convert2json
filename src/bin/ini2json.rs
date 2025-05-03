#![cfg(feature = "ini2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;
use serde_ini::de::from_read;

fn main() {
    parse_args()
        .map(from_read)
        .map(to_value)
        .for_each(stdout_writer);
}

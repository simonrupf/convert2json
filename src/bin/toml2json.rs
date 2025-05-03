#![cfg(feature = "toml2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::{to_value, toml::from_reader};

fn main() {
    parse_args()
        .map(from_reader)
        .map(to_value)
        .for_each(stdout_writer);
}

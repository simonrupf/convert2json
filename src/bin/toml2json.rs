#![cfg(feature = "toml2json")]
use convert2json::json::{parse_args, stdout_writer};
use convert2json::{string::from_reader, to_value};
use toml::from_str;

fn main() {
    parse_args()
        .map(|reader| from_str(&from_reader(reader)))
        .map(to_value)
        .for_each(stdout_writer);
}

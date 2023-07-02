extern crate serde_yaml;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::to_value;

#[cfg(feature = "yaml2json")]
fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&serde_yaml::from_reader(reader)));
    }
}

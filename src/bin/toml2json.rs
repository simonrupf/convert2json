extern crate toml;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::{string::from_reader, to_value};

#[cfg(feature = "toml2json")]
fn main() {
    for reader in parse_args() {
        stdout_writer(to_value(&toml::from_str(&from_reader(reader))));
    }
}

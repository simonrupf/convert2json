extern crate serde_yaml;
use convert2json::{stdin_reader, to_json};

#[cfg(feature = "yaml2json")]
fn main() {
    to_json(&serde_yaml::from_reader(stdin_reader()));
}

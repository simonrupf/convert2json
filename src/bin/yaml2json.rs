extern crate serde_yaml;
use convert2json::{stdin_reader, stdout_writer};

#[cfg(feature = "yaml2json")]
fn main() {
    stdout_writer(&serde_yaml::from_reader(stdin_reader()));
}

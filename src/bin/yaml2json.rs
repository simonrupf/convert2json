extern crate serde_yaml;
use convert2json::json::{parse_args, stdout_writer};
use convert2json::{stdin_reader, to_value};

#[cfg(feature = "yaml2json")]
fn main() {
    parse_args();
    stdout_writer(to_value(&serde_yaml::from_reader(stdin_reader())));
}

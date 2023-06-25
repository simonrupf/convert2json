extern crate serde_yaml;
use convert2json::{json, stdin_reader};

#[cfg(feature = "yaml2json")]
fn main() {
    json::parse_args();
    json::stdout_writer(&serde_yaml::from_reader(stdin_reader()));
}

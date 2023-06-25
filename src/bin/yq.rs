extern crate serde_yaml;
use convert2json::jq::{jq, parse_args, reader};

#[cfg(feature = "yq")]
fn main() {
    let (arguments, files) = parse_args();
    jq(&serde_yaml::from_reader(reader(&files)), &arguments);
}

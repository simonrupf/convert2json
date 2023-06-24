extern crate serde_yaml;
use convert2json::{parse_args, reader_from, to_jq};

#[cfg(feature = "yq")]
fn main() {
    let (arguments, files) = parse_args();
    to_jq(&serde_yaml::from_reader(reader_from(&files)), &arguments);
}

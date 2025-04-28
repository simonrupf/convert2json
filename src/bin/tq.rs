#![cfg(feature = "tq")]
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::{string::from_reader, to_value};
use toml::from_str;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(to_value(&from_str(&from_reader(reader))));
    }
}

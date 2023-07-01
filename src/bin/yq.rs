extern crate serde_yaml;
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::to_value;

#[cfg(feature = "yq")]
fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(to_value(&serde_yaml::from_reader(reader)));
    }
}

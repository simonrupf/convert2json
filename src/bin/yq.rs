extern crate serde_yaml;
use convert2json::jq::{parse_args, readers, Jq};

#[cfg(feature = "yq")]
fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(&serde_yaml::from_reader(reader));
    }
}

#![cfg(feature = "msgq")]
use convert2json::jq::{parse_args, readers, Jq};
use convert2json::to_value;
use rmp_serde::from_read;

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    for reader in readers(&files) {
        jq.write(to_value(&from_read(reader)));
    }
}

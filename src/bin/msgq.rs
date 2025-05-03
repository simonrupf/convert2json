#![cfg(feature = "msgq")]
use convert2json::jq::Jq;
use convert2json::to_value;
use rmp_serde::from_read;

fn main() {
    let mut jq = Jq::default();
    jq.readers()
        .map(from_read)
        .map(to_value)
        .for_each(|value| jq.write(value));
}

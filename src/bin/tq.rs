extern crate toml;
use convert2json::{jq::jq, jq::parse_args, string::from};

#[cfg(feature = "tq")]
fn main() {
    let (arguments, files) = parse_args();
    jq(&toml::from_str(&from(&files)), &arguments);
}

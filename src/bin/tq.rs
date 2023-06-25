extern crate toml;
use convert2json::{jq::parse_args, jq::Jq, string::from, string::from_stdin};

#[cfg(feature = "tq")]
fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    if files.is_empty() {
        jq.write(&toml::from_str(&from_stdin()));
    } else {
        for file in files {
            jq.write(&toml::from_str(&from(&file)));
        }
    }
}

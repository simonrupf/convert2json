#![cfg(feature = "tq")]
extern crate toml;
use convert2json::jq::{parse_args, Jq};
use convert2json::string::{from, from_reader};
use convert2json::{stdin_reader, to_value};

fn main() {
    let (arguments, files) = parse_args();
    let mut jq = Jq::new(&arguments);
    if files.is_empty() {
        jq.write(to_value(&toml::from_str(&from_reader(stdin_reader()))));
    } else {
        for file in files {
            jq.write(to_value(&toml::from_str(&from(&file))));
        }
    }
}

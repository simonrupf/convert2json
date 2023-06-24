extern crate toml;
use convert2json::lib::{parse_args, string_from, to_jq};

fn main() {
    let (arguments, files) = parse_args();
    to_jq(&toml::from_str(&string_from(&files)), &arguments);
}

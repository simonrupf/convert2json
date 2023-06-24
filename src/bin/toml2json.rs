extern crate toml;
use convert2json::{stdin_to_string, to_json};

#[cfg(feature = "toml2json")]
fn main() {
    let buffer = stdin_to_string();
    to_json(&toml::from_str(&buffer));
}

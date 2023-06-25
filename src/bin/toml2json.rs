extern crate toml;
use convert2json::{json, string::from_stdin};

#[cfg(feature = "toml2json")]
fn main() {
    json::parse_args();
    json::stdout_writer(&toml::from_str(&from_stdin()));
}

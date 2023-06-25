extern crate toml;
use convert2json::{stdout_writer, string::from_stdin};

#[cfg(feature = "toml2json")]
fn main() {
    stdout_writer(&toml::from_str(&from_stdin()));
}

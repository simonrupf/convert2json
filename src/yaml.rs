#![cfg(any(feature = "yaml", feature = "yaml2json", feature = "yq"))]
use super::{exit, to_value, Error};
use std::io::BufRead;
use yaml_split::DocumentIterator;

pub fn document_iterator<F>(readers: Vec<Box<dyn BufRead>>, function: F)
where
    F: Fn(&serde_json::Value),
{
    for reader in readers {
        for document in DocumentIterator::new(reader) {
            match document {
                Ok(document) => function(to_value(&serde_yaml::from_str(&document))),
                Err(e) => {
                    eprintln!("Error parsing input: {e}");
                    exit(Error::InputParsing as i32);
                }
            }
        }
    }
}

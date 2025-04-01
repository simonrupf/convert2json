#![cfg(any(feature = "yaml", feature = "yaml2json", feature = "yq"))]
use super::{exit, Error};
use std::io::BufRead;
use yaml_split::DocumentIterator;

pub fn document_iterator<F>(readers: Vec<Box<dyn BufRead>>, function: F)
where
    F: Fn(&serde_json::Value),
{
    for reader in readers {
        for document in DocumentIterator::new(reader) {
            match document {
                Ok(document) => {
                    let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&document) {
                        Ok(val) => val,
                        Err(e) => {
                            eprintln!("Error parsing input: {}", e.to_string());
                            exit(Error::InputParsing as i32);
                        }
                    };
                    function(&yaml_value_to_json_value(yaml_value))
                }
                Err(e) => {
                    eprintln!("Error parsing input: {e}");
                    exit(Error::InputParsing as i32);
                }
            }
        }
    }
}

fn yaml_value_to_json_value(value: serde_yaml::Value) -> serde_json::Value {
    match value {
        serde_yaml::Value::Null => serde_json::Value::Null,
        serde_yaml::Value::Bool(bool) => serde_json::Value::Bool(bool),
        serde_yaml::Value::Number(number) => {
            serde_json::Value::Number(yaml_number_to_json_number(number))
        }
        serde_yaml::Value::String(string) => serde_json::Value::String(string),
        serde_yaml::Value::Sequence(vec) => {
            serde_json::Value::Array(vec.into_iter().map(yaml_value_to_json_value).collect())
        }
        serde_yaml::Value::Mapping(mapping) => serde_json::Value::Object(
            mapping
                .into_iter()
                .map(|(key, val)| (yaml_value_to_json_key(key), yaml_value_to_json_value(val)))
                .collect(),
        ),
        serde_yaml::Value::Tagged(tagged_value) => yaml_value_to_json_value(tagged_value.value),
    }
}

fn yaml_number_to_json_number(number: serde_yaml::Number) -> serde_json::Number {
    if number.is_i64() {
        serde_json::Number::from_i128(number.as_i64().unwrap() as i128).unwrap()
    } else if number.is_u64() {
        serde_json::Number::from_u128(number.as_u64().unwrap() as u128).unwrap()
    } else {
        serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
    }
}

/// Convert a YAML value to a JSON object key.
/// JSON only allows strings as keys, but YAML mappings can have arbitrary values as keys.
fn yaml_value_to_json_key(value: serde_yaml::Value) -> String {
    value
        .as_str()
        .map(|str| str.to_string())
        .unwrap_or_else(|| serde_yaml::to_string(&value).unwrap())
}

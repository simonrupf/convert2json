#![cfg(any(feature = "yaml", feature = "yaml2json", feature = "yq"))]
use super::{exit, Error};
use std::io::BufRead;
use yaml_split::DocumentIterator;

pub fn document_iterator<F>(readers: Vec<Box<dyn BufRead>>, mut function: F)
where
    F: FnMut(&serde_json::Value),
{
    for reader in readers {
        for document in DocumentIterator::new(reader) {
            match document {
                Ok(document) => {
                    let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&document) {
                        Ok(val) => val,
                        Err(e) => {
                            eprintln!("Error parsing input: {e}");
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn test_read() {
        let input = r#"--- # The Smiths
- {name: John Smith, age: 33}
- name: Mary Smith
  age: 27
#- [name, age]: [Rae Smith, 4]
---
receipt:     Oz-Ware Purchase Invoice
date:        2012-08-06
customer:
    first_name:   Dorothy
    family_name:  Gale

items:
    - part_no:   A4786
      descrip:   Water Bucket (Filled)
      price:     1.47
      quantity:  4

    - part_no:   E1628
      descrip:   High Heeled "Ruby" Slippers
      size:      8
      price:     133.7
      quantity:  1

bill-to:  &id001
    street: |
            123 Tornado Alley
            Suite 16
    city:   East Centerville
    state:  KS

ship-to:  *id001

specialDelivery:  >
    Follow the Yellow Brick
    Road to the Emerald City.
    Pay no attention to the
    man behind the curtain.
...
---
singleton_map_style:
  a:
    Int: 2
  bs:
  - Int: 3
  c:
    Foo: bar
  d: 123
  e: 123
  f: False # No & no are interpreted as strings to avoid the Norway problem
---
tagged_style:
  a: !Int 0
  bs:
  - !Int 1
  c: !Foo "bar"
  d: !!float 123  # float, via explicit data type prefixed by (!!)
  e: !!str 123    # string, via explicit type
  f: !!str No     # string, not a boolean (Norway problem)"#;
        let readers: Vec<Box<dyn BufRead>> = vec![Box::new(input.as_bytes())];
        let mut results = vec![];
        document_iterator(readers, |value| {
            results.push(to_string(value));
        });
        assert_eq!(results.len(), 4);

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"{"tagged_style":{"a":0,"bs":[1],"c":"bar","d":123.0,"e":"123","f":"No"}}"#
        );

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"{"singleton_map_style":{"a":{"Int":2},"bs":[{"Int":3}],"c":{"Foo":"bar"},"d":123,"e":123,"f":false}}"#
        );

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"{"bill-to":{"city":"East Centerville","state":"KS","street":"123 Tornado Alley\nSuite 16\n"},"customer":{"family_name":"Gale","first_name":"Dorothy"},"date":"2012-08-06","items":[{"descrip":"Water Bucket (Filled)","part_no":"A4786","price":1.47,"quantity":4},{"descrip":"High Heeled \"Ruby\" Slippers","part_no":"E1628","price":133.7,"quantity":1,"size":8}],"receipt":"Oz-Ware Purchase Invoice","ship-to":{"city":"East Centerville","state":"KS","street":"123 Tornado Alley\nSuite 16\n"},"specialDelivery":"Follow the Yellow Brick Road to the Emerald City. Pay no attention to the man behind the curtain.\n"}"#
        );

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"[{"age":33,"name":"John Smith"},{"age":27,"name":"Mary Smith"}]"#
        );
    }
}

#![cfg(any(feature = "yaml", feature = "yaml2json", feature = "yq"))]
use super::{exit, Error};
use serde_json as j;
use serde_yaml as y;
use std::io::BufRead;
use yaml_split::DocumentIterator;

pub fn document_iterator<F>(readers: impl Iterator<Item = Box<dyn BufRead>>, mut function: F)
where
    F: FnMut(j::Value),
{
    for reader in readers {
        for document in DocumentIterator::new(reader) {
            match document {
                Ok(document) => {
                    let yaml_value = match y::from_str::<y::Value>(&document) {
                        Ok(val) => val,
                        Err(e) => {
                            eprintln!("Error parsing input: {e}");
                            exit(Error::InputParsing as i32);
                        }
                    };
                    function(yaml_to_json(&yaml_value))
                }
                Err(e) => {
                    eprintln!("Error parsing input: {e}");
                    exit(Error::InputParsing as i32);
                }
            }
        }
    }
}

fn yaml_to_json(value: &y::Value) -> j::Value {
    match value {
        y::Value::Null => j::Value::Null,
        y::Value::Bool(bool) => j::Value::Bool(*bool),
        y::Value::Number(number) => j::Value::Number(yaml_to_json_number(number)),
        y::Value::String(string) => j::Value::String(string.to_owned()),
        y::Value::Sequence(vec) => j::Value::Array(vec.iter().map(yaml_to_json).collect()),
        y::Value::Mapping(mapping) => j::Value::Object(
            mapping
                .iter()
                .map(|(key, val)| (yaml_to_json_key(key), yaml_to_json(val)))
                .collect(),
        ),
        y::Value::Tagged(tagged_value) => {
            let key = tagged_value
                .tag
                .to_string()
                .strip_prefix('!')
                .unwrap()
                .to_string();
            let mut map = j::Map::new();
            map.insert(key, yaml_to_json(&tagged_value.value));
            j::Value::Object(map)
        }
    }
}

fn yaml_to_json_number(number: &y::Number) -> j::Number {
    if number.is_i64() {
        j::Number::from_i128(number.as_i64().unwrap() as i128).unwrap()
    } else if number.is_u64() {
        j::Number::from_u128(number.as_u64().unwrap() as u128).unwrap()
    } else {
        j::Number::from_f64(number.as_f64().unwrap()).unwrap()
    }
}

/// Convert a YAML value to a JSON object key.
/// JSON only allows strings as keys, but YAML mappings can have arbitrary values as keys.
fn yaml_to_json_key(value: &y::Value) -> String {
    value
        .as_str()
        .map(|str| str.to_string())
        .unwrap_or_else(|| y::to_string(value).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        document_iterator(readers.into_iter(), |value| {
            results.push(j::to_string(&value));
        });
        assert_eq!(results.len(), 4);

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"{"tagged_style":{"a":{"Int":0},"bs":[{"Int":1}],"c":{"Foo":"bar"},"d":123.0,"e":"123","f":"No"}}"#
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
            r#"{"receipt":"Oz-Ware Purchase Invoice","date":"2012-08-06","customer":{"first_name":"Dorothy","family_name":"Gale"},"items":[{"part_no":"A4786","descrip":"Water Bucket (Filled)","price":1.47,"quantity":4},{"part_no":"E1628","descrip":"High Heeled \"Ruby\" Slippers","size":8,"price":133.7,"quantity":1}],"bill-to":{"street":"123 Tornado Alley\nSuite 16\n","city":"East Centerville","state":"KS"},"ship-to":{"street":"123 Tornado Alley\nSuite 16\n","city":"East Centerville","state":"KS"},"specialDelivery":"Follow the Yellow Brick Road to the Emerald City. Pay no attention to the man behind the curtain.\n"}"#
        );

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"[{"name":"John Smith","age":33},{"name":"Mary Smith","age":27}]"#
        );
    }
}

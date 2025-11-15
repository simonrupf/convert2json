#![cfg(any(feature = "yaml", feature = "yaml2json", feature = "yq"))]
use super::{exit, Error};
use serde_json as j;
use serde_saphyr as y;
use std::io::BufRead;

pub fn document_iterator<F>(readers: impl Iterator<Item = Box<dyn BufRead>>, mut function: F)
where
    F: FnMut(j::Value),
{
    let mut yaml: String = String::default();
    for mut reader in readers {
        match reader.read_to_string(&mut yaml) {
            Ok(_) => {
                let json_values = match y::from_multiple::<j::Value>(&yaml) {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error parsing input: {e}");
                        exit(Error::InputParsing as i32);
                    }
                };
                for json_value in json_values {
                    function(json_value);
                }
            }
            Err(e) => {
                eprintln!("Error parsing input: {e}");
                exit(Error::InputParsing as i32);
            }
        }
        yaml.clear();
    }
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
  f: No # No & no are interpreted as booleans, leading to the Norway problem
---
tagged_style:
  a: !Int 0
  bs:
  - !Int 1
  c: !Foo "bar"
#  d: !!float 123  # float, via explicit data type prefixed by (!!)
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
            // was r#"{"tagged_style":{"a":{"Int":0},"bs":[{"Int":1}],"c":{"Foo":"bar"},"d":123.0,"e":"123","f":"No"}}"#
            r#"{"tagged_style":{"a":0,"bs":[1],"c":"bar","e":123,"f":false}}"#
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

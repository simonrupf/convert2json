#![cfg(any(feature = "yaml", feature = "yaml2json", feature = "yq"))]
use super::{exit, to_value, Error};
use std::io::BufRead;
use yaml_split::DocumentIterator;

pub fn document_iterator<F>(readers: Vec<Box<dyn BufRead>>, mut function: F)
where
    F: FnMut(&serde_json::Value),
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
#  a: !Int 0
  bs:
#  - !Int 1
#  c: !Foo "bar"
  d: !!float 123  # float, via explicit data type prefixed by (!!)
  e: !!str 123    # string, via explicit type
  f: !!str No     # string, not a boolean (Norway problem)"#;
        let readers: Vec<Box<dyn BufRead>> = vec![Box::new(input.as_bytes())];
        let mut results = vec![];
        document_iterator(readers, |value| {results.push(to_string(value));});
        assert_eq!(results.len(), 4);

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"tagged_style":{"bs":null,"d":123.0,"e":"123","f":"No"}}"#);

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"singleton_map_style":{"a":{"Int":2},"bs":[{"Int":3}],"c":{"Foo":"bar"},"d":123,"e":123,"f":false}}"#);

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"bill-to":{"city":"East Centerville","state":"KS","street":"123 Tornado Alley\nSuite 16\n"},"customer":{"family_name":"Gale","first_name":"Dorothy"},"date":"2012-08-06","items":[{"descrip":"Water Bucket (Filled)","part_no":"A4786","price":1.47,"quantity":4},{"descrip":"High Heeled \"Ruby\" Slippers","part_no":"E1628","price":133.7,"quantity":1,"size":8}],"receipt":"Oz-Ware Purchase Invoice","ship-to":{"city":"East Centerville","state":"KS","street":"123 Tornado Alley\nSuite 16\n"},"specialDelivery":"Follow the Yellow Brick Road to the Emerald City. Pay no attention to the man behind the curtain.\n"}"#);

        let result = results.pop().unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"[{"age":33,"name":"John Smith"},{"age":27,"name":"Mary Smith"}]"#);
    }
}

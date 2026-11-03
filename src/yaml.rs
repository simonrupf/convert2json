#![cfg(any(feature = "yaml", feature = "yaml2json", feature = "yq"))]
use super::{Error, exit};
use pico_args::{Arguments, Error as picoError};
use serde::de::{Deserialize, Deserializer, Error as DeError, MapAccess, SeqAccess, Visitor};
use serde_json as j;
use serde_saphyr as y;
use std::ffi::OsString;
use std::fmt;
use std::io::BufRead;

/// Parses the YAML related command line arguments into parser options.
///
/// Prints the usage information to standard error, if requested, and exits
/// afterwards if `exit_on_help` is set.
#[must_use]
pub fn options(exit_on_help: bool) -> y::Options {
    match parse_options(Arguments::from_env(), exit_on_help) {
        Ok(options) => options,
        Err(e) => {
            eprintln!("Error {e}");
            exit(Error::ArgumentParsing as i32);
        }
    }
}

pub fn document_iterator<F>(
    readers: impl Iterator<Item = Box<dyn BufRead>>,
    options: &y::Options,
    mut function: F,
) where
    F: FnMut(j::Value),
{
    for mut reader in readers {
        for document in y::read_with_options::<_, Node>(&mut reader, options.clone()) {
            match document {
                Ok(Node(value)) => function(value),
                Err(e) => {
                    eprintln!("Error parsing input: {e}");
                    if matches!(e.without_snippet(), y::Error::Budget { .. }) {
                        eprintln!("Hint: the resource limits can be adjusted, see --help");
                    }
                    exit(Error::InputParsing as i32);
                }
            }
        }
    }
}

fn parse_options(mut pargs: Arguments, exit_on_help: bool) -> Result<y::Options, picoError> {
    let defaults = y::Budget::default();
    let budget = y::budget! {
        // documents get streamed, the remaining limits apply per document
        max_reader_input_bytes: None,
        max_depth: pargs
            .opt_value_from_str("--max-depth")?
            .unwrap_or(defaults.max_depth),
        max_nodes: pargs
            .opt_value_from_str("--max-nodes")?
            .unwrap_or(defaults.max_nodes),
        max_events: pargs
            .opt_value_from_str("--max-events")?
            .unwrap_or(defaults.max_events),
        max_total_scalar_bytes: pargs
            .opt_value_from_str("--max-scalar-bytes")?
            .unwrap_or(defaults.max_total_scalar_bytes),
    };
    let no_limits = pargs.contains("--no-limits");
    // pico-args doesn't support -help:
    // > short keys should be a single character or a repeated character
    if pargs.contains(["-h", "--help"])
        || pargs.contains("-?")
        || pargs.finish().contains(&OsString::from("-help"))
    {
        print_help(&defaults);
        if exit_on_help {
            exit(0);
        }
    }
    Ok(y::options! {
        budget: if no_limits { None } else { budget },
        // only true & false are booleans, not yes, no, on, off, etc. (Norway problem)
        strict_booleans: true,
        // keep !!binary values base64 encoded, as they may not be valid UTF-8
        ignore_binary_tag_for_string: true,
        // JSON can't represent these, so .nan, .inf & -.inf are kept as strings
        non_finite_float_policy: y::NonFiniteFloatPolicy::AsString,
    })
}

fn print_help(defaults: &y::Budget) {
    eprintln!(
        "\
Usage: yaml2json [--max-depth <n>] [--max-nodes <n>] [--max-events <n>] [--max-scalar-bytes <n>] [--no-limits] [files...]

Reads YAML from files or standard input and converts this to JSON, emitted on standard output. Any errors are reported to standard error and result in a non-zero exit code.

Options:
  --max-depth         maximum nesting depth of a document, defaults to: {}
  --max-nodes         maximum number of nodes (scalars, sequences & mappings) in
                      a document, defaults to: {}
  --max-events        maximum number of parser events in a document, defaults
                      to: {}
  --max-scalar-bytes  maximum combined size of all scalars in a document, in
                      bytes, defaults to: {}
  --no-limits         disable the above limits, alias expansion remains limited
                      to protect against \"billion laughs\" attacks
  -h, --help          display usage information
",
        defaults.max_depth,
        defaults.max_nodes,
        defaults.max_events,
        defaults.max_total_scalar_bytes,
    );
}

/// A YAML node, converted into a JSON value.
///
/// Values with a local tag, like `!Foo bar`, get wrapped in an object, using
/// the tag name as key: `{"Foo":"bar"}`. Tags that resolve to a URI, like the
/// YAML core schema ones (`!!str`, `!!float`, etc.), as well as the
/// non-specific tag `!`, only affect how the value gets parsed.
struct Node(j::Value);

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let y::Tagged(Untagged(value), tag) = y::Tagged::<Untagged>::deserialize(deserializer)?;
        let value = match tag.as_deref().and_then(|tag| tag.strip_prefix('!')) {
            Some(name) if !name.is_empty() => {
                j::Value::Object([(name.to_owned(), value)].into_iter().collect())
            }
            _ => value,
        };
        Ok(Self(value))
    }
}

/// The value of a YAML node, without its tag. Its children are tagged nodes.
struct Untagged(j::Value);

impl<'de> Deserialize<'de> for Untagged {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(UntaggedVisitor).map(Self)
    }
}

struct UntaggedVisitor;

impl<'de> Visitor<'de> for UntaggedVisitor {
    type Value = j::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any YAML value")
    }

    fn visit_bool<E: DeError>(self, value: bool) -> Result<Self::Value, E> {
        Ok(j::Value::Bool(value))
    }

    fn visit_i64<E: DeError>(self, value: i64) -> Result<Self::Value, E> {
        Ok(j::Value::from(value))
    }

    fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
        Ok(j::Value::from(value))
    }

    fn visit_f64<E: DeError>(self, value: f64) -> Result<Self::Value, E> {
        // non-finite floats get passed as strings, see parse_options()
        Ok(j::Number::from_f64(value).map_or(j::Value::Null, j::Value::Number))
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        Ok(j::Value::String(value.to_owned()))
    }

    fn visit_string<E: DeError>(self, value: String) -> Result<Self::Value, E> {
        Ok(j::Value::String(value))
    }

    fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
        Ok(j::Value::Null)
    }

    fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
        Ok(j::Value::Null)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Node::deserialize(deserializer).map(|Node(value)| value)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut values = Vec::new();
        while let Some(Node(value)) = seq.next_element()? {
            values.push(value);
        }
        Ok(j::Value::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut object = j::Map::new();
        while let Some((Node(key), Node(value))) = map.next_entry()? {
            object.insert(json_key(key), value);
        }
        Ok(j::Value::Object(object))
    }
}

/// Convert a YAML mapping key into a JSON object key.
/// JSON only allows strings as keys, but YAML mappings can have arbitrary
/// values as keys. These get serialized as JSON strings.
fn json_key(key: j::Value) -> String {
    match key {
        j::Value::String(string) => string,
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn options_from(arguments: &[&str]) -> y::Options {
        parse_options(
            Arguments::from_vec(arguments.iter().map(OsString::from).collect()),
            false,
        )
        .unwrap()
    }

    fn convert(input: &str, arguments: &[&str]) -> Result<Vec<j::Value>, y::Error> {
        y::read_with_options::<_, Node>(&mut input.as_bytes(), options_from(arguments))
            .map(|document| document.map(|Node(value)| value))
            .collect()
    }

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
        document_iterator(readers.into_iter(), &options_from(&[]), |value| {
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

    #[test]
    fn test_booleans() {
        assert_eq!(
            convert("[true, True, TRUE, false, False, FALSE]", &[]).unwrap(),
            vec![json!([true, true, true, false, false, false])]
        );
        assert_eq!(
            convert("[yes, no, on, off, y, n]", &[]).unwrap(),
            vec![json!(["yes", "no", "on", "off", "y", "n"])]
        );
    }

    #[test]
    fn test_tags() {
        let input = r"
local_scalar: !Point 1
local_sequence: !Polygon [1, 2]
local_mapping: !Point {x: 1, y: 2}
nested: !Outer [!Inner 1]
non_specific: ! 123
core_string: !!str 1.5
core_float: !!float 1
binary: !!binary aGVsbG8=
global: !<tag:example.com,2026:widget> value
";
        assert_eq!(
            convert(input, &[]).unwrap(),
            vec![json!({
                "local_scalar": {"Point": 1},
                "local_sequence": {"Polygon": [1, 2]},
                "local_mapping": {"Point": {"x": 1, "y": 2}},
                "nested": {"Outer": [{"Inner": 1}]},
                "non_specific": "123",
                "core_string": "1.5",
                "core_float": 1.0,
                "binary": "aGVsbG8=",
                "global": "value",
            })]
        );
    }

    #[test]
    fn test_keys() {
        let input = r"
200: integer
1.5: float
true: boolean
~: null
[a, b]: sequence
!Tag key: tagged
";
        assert_eq!(
            convert(input, &[]).unwrap(),
            vec![json!({
                "200": "integer",
                "1.5": "float",
                "true": "boolean",
                "null": null,
                r#"["a","b"]"#: "sequence",
                r#"{"Tag":"key"}"#: "tagged",
            })]
        );
    }

    #[test]
    fn test_non_finite_floats() {
        assert_eq!(
            convert("[.nan, .NaN, .inf, -.Inf, 1e999]", &[]).unwrap(),
            vec![json!([".nan", ".nan", ".inf", "-.inf", ".inf"])]
        );
    }

    #[test]
    fn test_merge_keys() {
        let input = r"
base: &base {a: 1, b: 2}
derived:
  <<: *base
  c: 3
";
        assert_eq!(
            convert(input, &[]).unwrap(),
            vec![json!({"base": {"a": 1, "b": 2}, "derived": {"a": 1, "b": 2, "c": 3}})]
        );
    }

    #[test]
    fn test_null_documents_skipped() {
        assert_eq!(
            convert("---\n~\n---\nfoo: bar\n---\n", &[]).unwrap(),
            vec![json!({"foo": "bar"})]
        );
    }

    #[test]
    fn test_limits() {
        let nested = "[[[1, 2, 3]]]";
        assert!(convert(nested, &[]).is_ok());
        for arguments in [
            ["--max-depth", "2"],
            ["--max-nodes", "5"],
            ["--max-events", "5"],
            ["--max-scalar-bytes", "2"],
        ] {
            let error = convert(nested, &arguments).unwrap_err();
            assert!(
                matches!(error.without_snippet(), y::Error::Budget { .. }),
                "{arguments:?}: {error}"
            );
        }
        assert_eq!(
            convert(nested, &["--max-nodes", "5", "--no-limits"]).unwrap(),
            vec![json!([[[1, 2, 3]]])]
        );
        let arguments = vec!["--max-depth".into(), "deep".into()];
        assert!(parse_options(Arguments::from_vec(arguments), false).is_err());
    }
}

#![cfg(any(feature = "xml", feature = "xml2json", feature = "xq"))]
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{Map, Value};
use std::io::BufRead;

pub fn wrap_xml_reader<R: BufRead>(reader: R) -> Value {
    let mut xml_reader = Reader::from_reader(reader);
    let config = xml_reader.config_mut();
    config.expand_empty_elements = true;
    config.trim_text(true);
    read(&mut xml_reader)
}

// TODO: change to $text to stay in line with quick_xml convention
const TEXT_NODE_KEY: &str = "#text";

/// This function is part of xmltojson.
///
/// xmltojson is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Lesser General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// xmltojson is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Lesser General Public License for more details.
///
/// You should have received a copy of the GNU LesserGeneral Public License
/// along with xmltojson.  If not, see <http://www.gnu.org/licenses/>.
///
/// See also: https://github.com/rtyler/xmltojson/ & https://crates.io/crates/xmltojson
///
/// Changes over the version of the function found in xmltojson:
/// - removed debug statements, to reduce required dependencies
/// - removed depth parameter, only used in debug statements
/// - handle duplicate nodes with attributes
fn read<R: BufRead>(reader: &mut Reader<R>) -> Value {
    let mut buf = Vec::new();
    let mut values = Vec::new();
    let mut node = Map::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if let Ok(name) = String::from_utf8(e.name().into_inner().to_vec()) {
                    let mut child = read(reader);
                    let mut attrs = Map::new();

                    let _ = e
                        .attributes()
                        .map(|a| {
                            if let Ok(attr) = a {
                                let key = String::from_utf8(attr.key.into_inner().to_vec());
                                let value = String::from_utf8(attr.value.to_vec());

                                // Only bother adding the attribute if both key and value are valid utf8
                                if let (Ok(key), Ok(value)) = (key, value) {
                                    let key = format!("@{}", key);
                                    let value = Value::String(value);

                                    // If the child is already an object, that's where the insert should happen
                                    if child.is_object() {
                                        child.as_object_mut().unwrap().insert(key, value);
                                    } else {
                                        attrs.insert(key, value);
                                    }
                                }
                            }
                        })
                        .collect::<Vec<_>>();

                    if node.contains_key(&name) {
                        let (_, mut existing) = node.remove_entry(&name).unwrap();
                        let mut entries: Vec<Value> = vec![];

                        if existing.is_array() {
                            let existing = existing.as_array_mut().unwrap();
                            while !existing.is_empty() {
                                entries.push(existing.remove(0));
                            }
                        } else {
                            entries.push(existing);
                        }

                        /*
                         * nodes with attributes need to be handled special
                         */
                        if !attrs.is_empty() {
                            if child.is_string() {
                                if let Some((_, value)) = attrs.remove_entry(TEXT_NODE_KEY) {
                                    if let Some(existing_str) = value.as_str() {
                                        let mut new_string = existing_str.to_string();
                                        new_string.push_str(child.as_str().unwrap());
                                        attrs.insert(
                                            TEXT_NODE_KEY.to_string(),
                                            Value::String(new_string),
                                        );
                                    } else {
                                        attrs.insert(TEXT_NODE_KEY.to_string(), child);
                                    }
                                } else {
                                    attrs.insert(TEXT_NODE_KEY.to_string(), child);
                                }
                            }

                            if let Ok(attrs) = serde_json::to_value(attrs) {
                                entries.push(attrs);
                            }
                        } else {
                            entries.push(child);
                        }

                        node.insert(name, Value::Array(entries));
                    /*
                     * nodes with attributes need to be handled special
                     */
                    } else if !attrs.is_empty() {
                        if child.is_string() {
                            attrs.insert(TEXT_NODE_KEY.to_string(), child);
                        }

                        if let Ok(attrs) = serde_json::to_value(attrs) {
                            node.insert(name, attrs);
                        }
                    } else {
                        node.insert(name, child);
                    }
                }
            }
            Ok(Event::Text(ref e)) => {
                if let Ok(decoded) = e.unescape() {
                    values.push(Value::String(decoded.to_string()));
                }
            }
            Ok(Event::CData(ref e)) => {
                if let Ok(decoded) = e.clone().escape() {
                    if let Ok(decoded_bt) = decoded.unescape() {
                        node.insert("#cdata".to_string(), Value::String(decoded_bt.to_string()));
                    }
                }
            }
            Ok(Event::End(ref _e)) => break,
            Ok(Event::Eof) => break,
            _ => (),
        }
    }

    if !node.is_empty() {
        // If we had collected some text along the way, that needs to be inserted
        // so we don't lose it
        let mut index = 0;
        let mut has_text = false;
        for value in values.iter() {
            if value.is_string() {
                has_text = true;
                break;
            }
            index += 1;
        }

        if has_text {
            node.insert("#text".to_string(), values.remove(index));
        }
        return serde_json::to_value(&node).expect("Failed to #to_value() a node!");
    }

    match values.len() {
        0 => Value::Null,
        1 => values.pop().unwrap(),
        _ => Value::Array(values),
    }
}

#[test]
fn test_read() {
    use serde_json::json;

    let input = r"";
    let result = read(&mut Reader::from_str(input));
    assert_eq!(result, Value::Null);

    // without config of expand_empty_elements true, empty node will be removed
    let input = r"<root/>";
    let result = read(&mut Reader::from_str(input));
    assert_eq!(result, Value::Null);

    let input = r"<key>value</key>";
    let result = read(&mut Reader::from_str(input));
    assert_eq!(result, json!({"key": "value"}));

    // without config of expand_empty_elements true, empty node will be removed
    let input = r#"<key attr="A">B</key><out>C<in/></out>"#;
    let result = read(&mut Reader::from_str(input));
    assert_eq!(
        result,
        json!({"key": {"#text": "B", "@attr": "A"}, "out": "C"})
    );

    let input = r"<tag><inner>A</inner><inner>B</inner></tag>";
    let result = read(&mut Reader::from_str(input));
    assert_eq!(result, json!({"tag": {"inner": ["A", "B"]}}));

    let input = r#"<tag><inner attr="A">A</inner><inner attr="B">B</inner></tag>"#;
    let result = read(&mut Reader::from_str(input));
    assert_eq!(
        result,
        json!({"tag": {"inner": [{"#text": "A", "@attr": "A"}, {"#text": "B", "@attr": "B"}]}})
    );

    // TODO: text appending
    let input = r"<tag>A <some>B</some> C</tag>";
    let result = read(&mut Reader::from_str(input));
    assert_eq!(result, json!({"tag": {"#text": "A  C", "some": "B"}}));

    // TODO: CData?
}

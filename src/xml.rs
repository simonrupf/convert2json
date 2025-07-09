#![cfg(any(feature = "xml", feature = "xml2json", feature = "xq"))]
use quick_xml::escape::resolve_predefined_entity;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{to_value, Map, Value};
use std::io::BufRead;
use std::mem::take;

pub fn wrap_xml_reader<R: BufRead>(reader: R) -> Value {
    let mut xml_reader = Reader::from_reader(reader);
    let config = xml_reader.config_mut();
    config.expand_empty_elements = true;
    // when trimming at the config level, we'd loose spaces between escaped entities
    read(&mut xml_reader)
}

trait AttrMap {
    fn insert_text(&mut self, value: &Value) -> Option<Value>;
    fn insert_text_node(&mut self, value: Value);
}

impl AttrMap for Map<String, Value> {
    fn insert_text(&mut self, value: &Value) -> Option<Value> {
        if !self.is_empty() {
            if value.is_string() {
                self.insert_text_node(value.clone());
            }
            if let Ok(attrs) = to_value(take(self)) {
                return Some(attrs);
            }
        }
        None
    }

    fn insert_text_node(&mut self, value: Value) {
        self.insert("$text".to_string(), value);
    }
}

struct NodeValues {
    node: Map<String, Value>,
    nodes: Vec<Map<String, Value>>,
    nodes_are_map: Vec<bool>,
    values: Vec<Value>,
}

impl NodeValues {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            node: Map::new(),
            nodes: Vec::new(),
            nodes_are_map: Vec::new(),
        }
    }

    fn insert(&mut self, key: String, value: Value) {
        self.node.insert(key, value);
    }

    fn insert_text(&mut self, text: &str) {
        if self.node.is_empty() {
            // if directly preceded by another string, append to it
            if let Some(value) = self.values.pop() {
                let mut value_text = value.as_str().unwrap_or_default().to_string();
                value_text.push_str(text);
                self.values.push(Value::String(value_text));
                return;
            }
        } else {
            // don't insert whitespace between nodes
            if text.trim().is_empty() {
                return;
            }

            self.nodes.push(take(&mut self.node));
            self.nodes_are_map.push(true);
        }

        self.values.push(Value::String(text.to_string()));
        self.nodes_are_map.push(false);
    }

    fn remove_entry(&mut self, key: &String) -> Option<Value> {
        if self.node.contains_key(key) {
            if let Some((_, existing)) = self.node.remove_entry(key) {
                return Some(existing);
            }
        }
        None
    }

    fn get_value(&mut self) -> Value {
        if !self.node.is_empty() {
            self.nodes.push(take(&mut self.node));
            self.nodes_are_map.push(true);
        }

        if !self.nodes.is_empty() {
            // If we had collected some non-whitespace text along the way, that
            // needs to be inserted so we don't lose it

            if self.nodes.len() == 1 && self.values.len() <= 1 {
                if self.values.len() == 1 {
                    let value = self.values.remove(0);
                    let text = value.as_str().unwrap_or_default().trim();
                    if !text.is_empty() {
                        self.nodes[0].insert_text_node(Value::String(text.to_string()));
                    }
                }
                return to_value(&self.nodes[0]).expect("Failed to #to_value() a node!");
            }
            for (index, node_is_map) in self.nodes_are_map.iter().enumerate() {
                if *node_is_map {
                    self.values
                        .insert(index, Value::Object(self.nodes.remove(0)));
                }
            }
        }

        // trim any values left, removing empty strings
        self.values = self
            .values
            .clone()
            .into_iter()
            .filter_map(|value| {
                if value.is_string() {
                    let trimmed = value.as_str().unwrap_or_default().trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    return Some(Value::String(trimmed.to_string()));
                }
                Some(value)
            })
            .collect();

        match self.values.len() {
            0 => Value::Null,
            1 => self.values.pop().unwrap(),
            _ => Value::Array(take(&mut self.values)),
        }
    }
}

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
/// You should have received a copy of the GNU Lesser General Public License
/// along with xmltojson.  If not, see <http://www.gnu.org/licenses/>.
///
/// See also: <https://github.com/rtyler/xmltojson>/ & <https://crates.io/crates/xmltojson>
///
/// Changes over the version of the function found in xmltojson:
/// - removed debug statements, to reduce required dependencies
/// - removed depth parameter, only used in debug statements
/// - CDATA gets treated as text, not preserved as an object with #cdata key
fn read<R: BufRead>(reader: &mut Reader<R>) -> Value {
    let mut buf = Vec::new();
    let mut nodes = NodeValues::new();

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
                                    let key = format!("@{key}");
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

                    if let Some(mut existing) = nodes.remove_entry(&name) {
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
                        if let Some(attrs) = attrs.insert_text(&child) {
                            entries.push(attrs);
                        } else {
                            entries.push(child);
                        }

                        nodes.insert(name, Value::Array(entries));
                    /*
                     * nodes with attributes need to be handled special
                     */
                    } else if let Some(attrs) = attrs.insert_text(&child) {
                        nodes.insert(name, attrs);
                    } else {
                        nodes.insert(name, child);
                    }
                }
            }
            Ok(Event::Text(ref e)) => {
                if let Ok(decoded) = e.decode() {
                    nodes.insert_text(&decoded);
                }
            }
            Ok(Event::CData(ref e)) => {
                if let Ok(decoded) = e.decode() {
                    nodes.insert_text(&decoded);
                }
            }
            Ok(Event::GeneralRef(ref e)) => {
                if let Ok(Some(ch)) = e.resolve_char_ref() {
                    nodes.insert_text(&ch.to_string());
                } else if let Ok(decoded) = e.decode() {
                    if let Some(entity) = resolve_predefined_entity(&decoded) {
                        nodes.insert_text(entity);
                    }
                }
            }
            Ok(Event::End(ref _e)) => break,
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
    nodes.get_value()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_read() {
        let input = r"";
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, Value::Null);

        // without config of expand_empty_elements true, empty node will be removed
        let input = r"<root/>";
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, Value::Null);

        let mut reader = Reader::from_str(input);
        let config = reader.config_mut();
        config.expand_empty_elements = true;
        let result = read(&mut reader);
        assert_eq!(result, json!({"root": null}));

        let input = r"<key>value</key>";
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, json!({"key": "value"}));

        // without config of expand_empty_elements true, empty node will be removed
        let input = r#"<key attr="A">B</key><out>C<in/></out>"#;
        let result = read(&mut Reader::from_str(input));
        assert_eq!(
            result,
            json!({"key": {"$text": "B", "@attr": "A"}, "out": "C"})
        );

        let mut reader = Reader::from_str(input);
        let config = reader.config_mut();
        config.expand_empty_elements = true;
        let result = read(&mut reader);
        assert_eq!(
            result,
            json!({"key": {"$text": "B", "@attr": "A"}, "out": {"$text": "C", "in": null}})
        );

        let input = r"<tag><inner>A</inner><inner>B</inner></tag>";
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, json!({"tag": {"inner": ["A", "B"]}}));

        let input = r#"<tag><inner attr="A">A</inner><inner attr="B">B</inner></tag>"#;
        let result = read(&mut Reader::from_str(input));
        assert_eq!(
            result,
            json!({"tag": {"inner": [{"$text": "A", "@attr": "A"}, {"$text": "B", "@attr": "B"}]}})
        );

        // without config of expand_empty_elements true, empty node will be removed
        let input = r#"<tag>A <some attr="B"/> C</tag>"#;
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, json!({"tag": "A  C"}));

        let mut reader = Reader::from_str(input);
        let config = reader.config_mut();
        config.expand_empty_elements = true;
        let result = read(&mut reader);
        assert_eq!(
            result,
            json!({"tag": ["A", {"some": {"@attr": "B"}}, "C"]})
        );

        let input = r"<tag>A <some>B</some> C <some>D</some></tag>";
        let result = read(&mut Reader::from_str(input));
        assert_eq!(
            result,
            json!({"tag": ["A", {"some": "B"}, "C", {"some": "D"}]})
        );

        let input = r"<![CDATA[sample]]>";
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, json!("sample"));

        let input = r"<tag><![CDATA[sample]]></tag>";
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, json!({"tag": "sample"}));

        let input = r#"<tag attr="B"><![CDATA[A]]></tag>"#;
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, json!({"tag": {"$text": "A", "@attr": "B"}}));

        let input = r#"<tag attr="C">A <some><![CDATA[B]]></some></tag>"#;
        let result = read(&mut Reader::from_str(input));
        assert_eq!(
            result,
            json!({"tag": {"some": "B", "$text": "A", "@attr": "C"}})
        );

        let input = r"<pets>A cat &amp; a dog</pets>";
        let result = read(&mut Reader::from_str(input));
        assert_eq!(result, json!({"pets": "A cat & a dog"}));
    }
}

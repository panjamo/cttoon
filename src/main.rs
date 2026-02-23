use std::collections::BTreeMap;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use format_as_toon::{encode_toon, Delimiter, KeyFolding, ToonOptions};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde_json::{Map, Value};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum DelimiterArg {
    Comma,
    Tab,
    Pipe,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum KeyFoldingArg {
    Off,
    Safe,
}

/// Convert JSON or XML to TOON (Token-Oriented Object Notation).
///
/// Reads JSON or XML from a file or stdin (auto-detected) and outputs TOON to stdout.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Input file (JSON or XML; reads from stdin if omitted)
    input: Option<PathBuf>,

    /// Delimiter for array values and tabular rows
    #[arg(short, long, value_enum, default_value_t = DelimiterArg::Comma)]
    delimiter: DelimiterArg,

    /// Number of spaces per indentation level
    #[arg(short, long, default_value_t = 2)]
    spaces: usize,

    /// Key folding mode (collapse single-key chains into dotted paths)
    #[arg(short, long, value_enum, default_value_t = KeyFoldingArg::Off)]
    key_folding: KeyFoldingArg,

    /// Maximum depth for key folding (default: unlimited)
    #[arg(short, long)]
    flatten_depth: Option<usize>,
}

/// Recursively parse XML events into a serde_json::Value.
///
/// Returns the parsed value and any trailing text content found after child elements.
fn parse_xml_element(
    reader: &mut Reader<&[u8]>,
    tag_name: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut fields: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut text_content = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let child_name = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                // collect attributes on child element
                let mut attr_map: BTreeMap<String, Vec<Value>> = BTreeMap::new();
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = format!(
                        "@{}",
                        String::from_utf8_lossy(attr.key.local_name().as_ref())
                    );
                    let val = Value::String(String::from_utf8_lossy(&attr.value).into_owned());
                    attr_map.entry(key).or_default().push(val);
                }
                let child_val = parse_xml_element(reader, &child_name)?;
                // merge attributes into child if it's an object
                let child_val = if !attr_map.is_empty() {
                    let mut obj = match child_val {
                        Value::Object(m) => m,
                        other => {
                            let mut m = Map::new();
                            if !matches!(other, Value::Null) {
                                m.insert("#text".to_string(), other);
                            }
                            m
                        }
                    };
                    for (k, mut vals) in attr_map {
                        let v = if vals.len() == 1 {
                            vals.remove(0)
                        } else {
                            Value::Array(vals)
                        };
                        obj.insert(k, v);
                    }
                    Value::Object(obj)
                } else {
                    child_val
                };
                fields.entry(child_name).or_default().push(child_val);
            }
            Event::Empty(e) => {
                let child_name = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                let mut obj = Map::new();
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = format!(
                        "@{}",
                        String::from_utf8_lossy(attr.key.local_name().as_ref())
                    );
                    let val = Value::String(String::from_utf8_lossy(&attr.value).into_owned());
                    obj.insert(key, val);
                }
                let child_val = if obj.is_empty() {
                    Value::Null
                } else {
                    Value::Object(obj)
                };
                fields.entry(child_name).or_default().push(child_val);
            }
            Event::Text(e) => {
                let t = e.unescape()?.into_owned();
                let trimmed = t.trim().to_string();
                if !trimmed.is_empty() {
                    text_content.push_str(&trimmed);
                }
            }
            Event::CData(e) => {
                text_content.push_str(&String::from_utf8_lossy(e.as_ref()));
            }
            Event::End(e) => {
                let ended = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                if ended == tag_name {
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    if fields.is_empty() {
        // leaf node
        if text_content.is_empty() {
            return Ok(Value::Null);
        }
        return Ok(Value::String(text_content));
    }

    let mut obj = Map::new();
    if !text_content.is_empty() {
        obj.insert("#text".to_string(), Value::String(text_content));
    }
    for (key, mut vals) in fields {
        let v = if vals.len() == 1 {
            vals.remove(0)
        } else {
            Value::Array(vals)
        };
        obj.insert(key, v);
    }
    Ok(Value::Object(obj))
}

fn xml_to_json(xml: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    // Find the root element
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let root_name = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                // collect attributes on root
                let mut attr_map: BTreeMap<String, Vec<Value>> = BTreeMap::new();
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = format!(
                        "@{}",
                        String::from_utf8_lossy(attr.key.local_name().as_ref())
                    );
                    let val = Value::String(String::from_utf8_lossy(&attr.value).into_owned());
                    attr_map.entry(key).or_default().push(val);
                }
                let inner = parse_xml_element(&mut reader, &root_name)?;
                let inner = if !attr_map.is_empty() {
                    let mut obj = match inner {
                        Value::Object(m) => m,
                        other => {
                            let mut m = Map::new();
                            if !matches!(other, Value::Null) {
                                m.insert("#text".to_string(), other);
                            }
                            m
                        }
                    };
                    for (k, mut vals) in attr_map {
                        let v = if vals.len() == 1 {
                            vals.remove(0)
                        } else {
                            Value::Array(vals)
                        };
                        obj.insert(k, v);
                    }
                    Value::Object(obj)
                } else {
                    inner
                };
                let mut root_obj = Map::new();
                root_obj.insert(root_name, inner);
                return Ok(Value::Object(root_obj));
            }
            Event::Empty(e) => {
                let root_name = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                let mut obj = Map::new();
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = format!(
                        "@{}",
                        String::from_utf8_lossy(attr.key.local_name().as_ref())
                    );
                    let val = Value::String(String::from_utf8_lossy(&attr.value).into_owned());
                    obj.insert(key, val);
                }
                let child_val = if obj.is_empty() {
                    Value::Null
                } else {
                    Value::Object(obj)
                };
                let mut root_obj = Map::new();
                root_obj.insert(root_name, child_val);
                return Ok(Value::Object(root_obj));
            }
            Event::Eof => return Err("empty XML document".into()),
            _ => {}
        }
        buf.clear();
    }
}

fn parse_input(input: &str) -> Result<Value, Box<dyn std::error::Error>> {
    if input.trim_start().starts_with('<') {
        xml_to_json(input)
    } else {
        Ok(serde_json::from_str(input)?)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let raw_input = match &args.input {
        Some(path) => std::fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let value = parse_input(&raw_input)?;

    let opts = ToonOptions {
        delimiter: match args.delimiter {
            DelimiterArg::Comma => Delimiter::Comma,
            DelimiterArg::Tab => Delimiter::Tab,
            DelimiterArg::Pipe => Delimiter::Pipe,
        },
        indent: args.spaces,
        key_folding: match args.key_folding {
            KeyFoldingArg::Off => KeyFolding::Off,
            KeyFoldingArg::Safe => KeyFolding::Safe,
        },
        flatten_depth: args.flatten_depth.unwrap_or(usize::MAX),
    };

    let output = encode_toon(&value, &opts);
    print!("{output}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_passthrough() {
        let v = parse_input(r#"{"name":"Alice","age":30}"#).unwrap();
        assert_eq!(v["name"], json!("Alice"));
        assert_eq!(v["age"], json!(30));
    }

    #[test]
    fn test_xml_simple() {
        let xml = "<person><name>Alice</name><age>30</age></person>";
        let v = parse_input(xml).unwrap();
        assert_eq!(v["person"]["name"], json!("Alice"));
        assert_eq!(v["person"]["age"], json!("30"));
    }

    #[test]
    fn test_xml_with_attributes() {
        let xml = r#"<item id="1"><label>foo</label></item>"#;
        let v = parse_input(xml).unwrap();
        assert_eq!(v["item"]["@id"], json!("1"));
        assert_eq!(v["item"]["label"], json!("foo"));
    }

    #[test]
    fn test_xml_repeated_elements_become_array() {
        let xml = "<root><item>a</item><item>b</item></root>";
        let v = parse_input(xml).unwrap();
        assert_eq!(v["root"]["item"], json!(["a", "b"]));
    }

    #[test]
    fn test_xml_with_declaration() {
        let xml = r#"<?xml version="1.0"?><root><val>42</val></root>"#;
        let v = parse_input(xml).unwrap();
        assert_eq!(v["root"]["val"], json!("42"));
    }
}

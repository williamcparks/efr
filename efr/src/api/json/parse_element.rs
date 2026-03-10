use quick_xml::{
    NsReader,
    events::{BytesStart, Event},
};
use serde_json::Value;

use crate::api::EfrError;

use super::{
    insert_or_arrayify::insert_or_arrayify, is_xsi_nil::is_xsi_nil, json_key::json_key,
    parse_attributes::parse_attributes,
};

pub fn parse_element(reader: &mut NsReader<&[u8]>, start: &BytesStart) -> Result<Value, EfrError> {
    let mut map = parse_attributes(start, reader.resolver())?;
    let mut text_buf = String::new();

    loop {
        match reader.read_resolved_event()? {
            (resolve_result, Event::Start(child)) => {
                let key = json_key(child.local_name().as_ref(), &resolve_result);
                let is_null = is_xsi_nil(&child, reader.resolver())?;

                let value = if is_null {
                    reader.read_to_end(child.to_end().name())?;
                    Value::Null
                } else {
                    parse_element(reader, &child)?
                };

                insert_or_arrayify(&mut map, key, value);
            }
            (resolve_result, Event::Empty(child)) => {
                let key = json_key(child.local_name().as_ref(), &resolve_result);
                let is_null = is_xsi_nil(&child, reader.resolver())?;

                let value = if is_null {
                    Value::Null
                } else {
                    let attrs = parse_attributes(&child, reader.resolver())?;
                    if attrs.is_empty() {
                        Value::Null
                    } else {
                        Value::Object(attrs)
                    }
                };

                insert_or_arrayify(&mut map, key, value);
            }
            (_, Event::Text(text)) => {
                let content = text.xml10_content()?;
                text_buf.push_str(content.trim());
            }
            (_, Event::End(_)) | (_, Event::Eof) => break,
            _ => {}
        }
    }

    Ok(if !map.is_empty() {
        if !text_buf.is_empty() {
            map.insert("$text".to_owned(), Value::String(text_buf));
        }
        Value::Object(map)
    } else if !text_buf.is_empty() {
        Value::String(text_buf.trim().to_owned())
    } else {
        Value::Null
    })
}

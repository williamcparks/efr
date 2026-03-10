use quick_xml::{NsReader, events::Event, name::ResolveResult};
use serde_json::Value;

use crate::api::EfrError;

use super::{
    insert_or_arrayify::insert_or_arrayify, is_xsi_nil::is_xsi_nil, json_key::json_key,
    parse_attributes::parse_attributes, parse_element::parse_element,
};

pub fn handler(xml: &str) -> Result<Value, EfrError> {
    let mut reader = NsReader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut in_body = false;
    let mut result_map = serde_json::Map::new();

    loop {
        match reader.read_resolved_event()? {
            (ResolveResult::Bound(ns), Event::Start(start))
                if ns.as_ref() == SOAP_ENVELOPE && start.local_name().as_ref() == b"Body" =>
            {
                in_body = true;
            }
            (resolve_result, Event::Start(start)) if in_body => {
                let key = json_key(start.local_name().as_ref(), &resolve_result);
                let is_null = is_xsi_nil(&start, reader.resolver())?;

                let value = if is_null {
                    reader.read_to_end(start.to_end().name())?;
                    Value::Null
                } else {
                    parse_element(&mut reader, &start)?
                };

                insert_or_arrayify(&mut result_map, key, value);
            }
            (resolve_result, Event::Empty(start)) if in_body => {
                let key = json_key(start.local_name().as_ref(), &resolve_result);
                let is_null = is_xsi_nil(&start, reader.resolver())?;

                let value = if is_null {
                    Value::Null
                } else {
                    let attrs = parse_attributes(&start, reader.resolver())?;
                    if attrs.is_empty() {
                        Value::Null
                    } else {
                        Value::Object(attrs)
                    }
                };

                insert_or_arrayify(&mut result_map, key, value);
            }
            (ResolveResult::Bound(ns), Event::End(end))
                if ns.as_ref() == SOAP_ENVELOPE && end.local_name().as_ref() == b"Body" =>
            {
                break;
            }
            (_, Event::Eof) => break,
            _ => {}
        }
    }

    Ok(if result_map.is_empty() {
        Value::Null
    } else {
        Value::Object(result_map)
    })
}

pub const SOAP_ENVELOPE: &[u8] = b"http://schemas.xmlsoap.org/soap/envelope/";

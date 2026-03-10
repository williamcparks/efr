use quick_xml::{
    events::BytesStart,
    name::{NamespaceResolver, Prefix, ResolveResult},
};
use serde_json::{Map, Value};

use crate::api::EfrError;

use super::{is_xsi_nil::XSI, json_key::json_key_attr};

pub fn parse_attributes(
    start: &BytesStart,
    resolver: &NamespaceResolver,
) -> Result<Map<String, Value>, EfrError> {
    let mut map = Map::new();

    for attr_result in start.attributes() {
        let attr = attr_result?;

        if attr
            .key
            .prefix()
            .as_ref()
            .map(Prefix::is_xmlns)
            .unwrap_or_default()
        {
            continue;
        }
        if attr.key.local_name().as_ref() == b"xmlns" {
            continue;
        }

        let (resolve_result, local_name) = resolver.resolve_attribute(attr.key);

        if let ResolveResult::Bound(ns) = resolve_result
            && ns.0 == XSI
            && local_name.as_ref() == b"nil"
        {
            continue;
        }

        let key = json_key_attr(local_name.as_ref(), &resolve_result);

        let value = Value::String(attr.unescape_value()?.into_owned());

        map.insert(key, value);
    }

    Ok(map)
}

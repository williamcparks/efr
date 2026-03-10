use quick_xml::{
    events::BytesStart,
    name::{NamespaceResolver, ResolveResult},
};

use crate::api::EfrError;

pub fn is_xsi_nil(start: &BytesStart, resolver: &NamespaceResolver) -> Result<bool, EfrError> {
    for attr in start.attributes() {
        let attr = attr?;

        let (ns, local_name) = match resolver.resolve_attribute(attr.key) {
            (ResolveResult::Bound(ns), local) => (ns, local),
            (ResolveResult::Unbound | ResolveResult::Unknown(_), _) => continue,
        };

        if ns.0 != XSI {
            continue;
        }
        if local_name.as_ref() != b"nil" {
            continue;
        }
        let value = attr.value.as_ref();

        return Ok(value == b"true" || value == b"1");
    }

    Ok(false)
}

pub const XSI: &[u8] = b"http://www.w3.org/2001/XMLSchema-instance";

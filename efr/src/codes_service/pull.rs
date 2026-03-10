use std::fmt::Display;

use quick_xml::{
    NsReader,
    events::{BytesStart, Event},
    name::ResolveResult,
};

use crate::codes_service::EfrCodesError;

pub struct Tag {
    local_name: &'static str,
    uri: Option<&'static str>,
}

pub struct Pull<'a, 'b>(pub &'b mut NsReader<&'a [u8]>);

impl<'a, 'b> Pull<'a, 'b> {
    pub fn open_row(&mut self) -> Result<bool, EfrCodesError> {
        match self.0.read_event()? {
            Event::Start(start) => {
                let local_name = start.local_name();
                match local_name.as_ref() {
                    b"Row" => Ok(true),
                    _ => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Expected <Row> or </SimpleCodeList>. Got {start:?}"
                    )))),
                }
            }
            Event::End(end) => {
                let local_name = end.local_name();
                match local_name.as_ref() {
                    b"SimpleCodeList" => Ok(false),
                    _ => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Expected <Row> or </SimpleCodeList>. Got {end:?}"
                    )))),
                }
            }
            Event::Eof => Err(EfrCodesError::Xml(quick_xml::DeError::UnexpectedEof)),
            other => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                "Expected <Row> or </SimpleCodeList>. Got {other:?}"
            )))),
        }
    }

    pub fn open(&mut self, tag: Tag) -> Result<BytesStart<'a>, EfrCodesError> {
        match self.0.read_event()? {
            Event::Start(start) => {
                let (resolve_result, local_name) = self.0.resolver().resolve_element(start.name());

                if local_name.as_ref() != tag.local_name.as_bytes() {
                    return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Expected {tag}. Got {start:?}"
                    ))));
                }

                match (resolve_result, tag.uri) {
                    (ResolveResult::Unbound, None) => Ok(start),
                    (ResolveResult::Bound(ns), Some(uri)) if uri.as_bytes() == ns.0 => Ok(start),
                    _ => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Expected {tag}. Got {start:?}"
                    )))),
                }
            }
            Event::Eof => Err(EfrCodesError::Xml(quick_xml::DeError::UnexpectedEof)),
            other => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                "Expected {tag}. Got {other:?}"
            )))),
        }
    }

    pub fn xml_decl(&mut self) -> Result<(), EfrCodesError> {
        match self.0.read_event()? {
            Event::Decl(_) => Ok(()),
            event => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                "Expected XML Declaration Got: {event:#?}"
            )))),
        }
    }
}

impl Tag {
    pub const fn new(local_name: &'static str, uri: &'static str) -> Self {
        Self {
            local_name,
            uri: Some(uri),
        }
    }

    pub const fn local(local_name: &'static str) -> Self {
        Self {
            local_name,
            uri: None,
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let local_name = self.local_name;
        match self.uri {
            Some(uri) => write!(f, "<{local_name} xmlns=\"{uri}\">"),
            None => write!(f, "<{local_name}>"),
        }
    }
}

use std::borrow::Cow;

use bitflags::bitflags;
use efr_macros::CodeList;
use quick_xml::NsReader;
use serde::{Deserialize, Serialize};

use crate::codes_service::EfrCodesError;

use super::utils::CodeRow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeDataField<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub helptext: Cow<'a, str>,
    pub ghosttext: Cow<'a, str>,
    pub contextualhelpdata: Cow<'a, str>,
    pub validationmessage: Cow<'a, str>,
    pub regularexpression: Cow<'a, str>,
    pub defaultvalueexpression: Cow<'a, str>,
    pub flags: CodeDataFieldFlags,
}

bitflags! {
    #[derive(Clone, Debug)]
    pub struct CodeDataFieldFlags: u8 {
        const IS_VISIBLE = 1 << 0;
        const IS_REQUIRED = 1 << 1;
        const IS_READ_ONLY = 1 << 2;
    }
}

impl<'a> CodeRow<'a> for CodeDataField<'a> {
    fn code_row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError> {
        let raw = match RawCodeDataField::code_row(ns_reader) {
            Ok(Some(some)) => some,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        let mut flags = CodeDataFieldFlags::empty();
        if raw.isvisible == "True" {
            flags |= CodeDataFieldFlags::IS_VISIBLE;
        }
        if raw.isrequired == "True" {
            flags |= CodeDataFieldFlags::IS_REQUIRED;
        }
        if raw.isreadonly == "True" {
            flags |= CodeDataFieldFlags::IS_READ_ONLY;
        }

        Ok(Some(Self {
            code: raw.code,
            name: raw.name,
            helptext: raw.helptext,
            ghosttext: raw.ghosttext,
            contextualhelpdata: raw.contextualhelpdata,
            validationmessage: raw.validationmessage,
            regularexpression: raw.regularexpression,
            defaultvalueexpression: raw.defaultvalueexpression,
            flags,
        }))
    }
}

#[derive(CodeList)]
#[codelist("Code Data Field")]
struct RawCodeDataField<'a> {
    code: Cow<'a, str>,
    #[codelist(optional)]
    name: Cow<'a, str>,
    #[codelist(optional)]
    isvisible: Cow<'a, str>,
    #[codelist(optional)]
    isrequired: Cow<'a, str>,
    #[codelist(optional)]
    helptext: Cow<'a, str>,
    #[codelist(optional)]
    ghosttext: Cow<'a, str>,
    #[codelist(optional)]
    contextualhelpdata: Cow<'a, str>,
    #[codelist(optional)]
    validationmessage: Cow<'a, str>,
    #[codelist(optional)]
    regularexpression: Cow<'a, str>,
    #[codelist(optional)]
    defaultvalueexpression: Cow<'a, str>,
    #[codelist(optional)]
    isreadonly: Cow<'a, str>,
}

impl Serialize for CodeDataFieldFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

impl<'de> Deserialize<'de> for CodeDataFieldFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(bits))
    }
}

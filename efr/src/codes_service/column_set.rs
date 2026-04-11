use std::borrow::Cow;

use quick_xml::Reader;
use serde::Deserialize;

use crate::codes_service::{EfrCodesError, extract::extract};

#[derive(Deserialize)]
pub(super) struct ColumnSet<'a> {
    #[serde(rename = "Column", default)]
    pub(super) columns: Vec<Column<'a>>,
}

#[derive(Deserialize)]
pub(super) struct Column<'a> {
    #[serde(rename = "@Id")]
    pub(super) id: Cow<'a, str>,

    #[serde(rename = "@Use", default)]
    pub(super) use_: ColumnUse,
}

#[derive(Clone, Copy, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(super) enum ColumnUse {
    #[default]
    Optional,
    Required,
}

impl<'a> ColumnSet<'a> {
    pub(super) fn try_new(
        xml: &'a str,
        reader: &mut Reader<&'_ [u8]>,
    ) -> Result<Self, EfrCodesError> {
        match extract("ColumnSet", xml, reader)? {
            Some(xml) => Ok(quick_xml::de::from_str(xml)?),
            None => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(
                "Expected `ColumnSet` In Code Set List".to_owned(),
            ))),
        }
    }
}

impl ColumnUse {
    pub fn is_required(self) -> bool {
        self == Self::Required
    }
}

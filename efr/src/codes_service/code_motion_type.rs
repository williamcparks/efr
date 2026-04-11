use std::borrow::Cow;

use efr_macros::CodeList;
use quick_xml::NsReader;
use serde::{Deserialize, Serialize};

use crate::codes_service::{EfrCodesError, utils::CodeRow};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeMotionType<'a> {
    pub code: Cow<'a, str>,
}

impl<'a> CodeRow<'a> for CodeMotionType<'a> {
    fn code_row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError> {
        let raw = match RawCodeMotionType::code_row(ns_reader) {
            Ok(Some(some)) => some,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        Ok(Some(Self { code: raw.code }))
    }
}

#[derive(CodeList)]
#[codelist("Code Motion Type")]
struct RawCodeMotionType<'a> {
    code: Cow<'a, str>,
}

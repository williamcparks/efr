use std::borrow::Cow;

use efr_macros::CodeList;
use quick_xml::NsReader;
use serde::{Deserialize, Serialize};

use crate::codes_service::EfrCodesError;

use super::utils::CodeRow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeError<'a> {
    pub code: i64,
    pub name: Cow<'a, str>,
}

impl<'a> CodeRow<'a> for CodeError<'a> {
    fn code_row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError> {
        let raw = match RawCodeError::code_row(ns_reader) {
            Ok(Some(some)) => some,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        let code = match raw.code.parse() {
            Ok(ok) => ok,
            Err(err) => {
                return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                    "Failed To Parse `code`: `{err}` Got: `{}`",
                    raw.code
                ))));
            }
        };

        Ok(Some(Self {
            code,
            name: raw.name,
        }))
    }
}

#[derive(CodeList)]
#[codelist("Code Error")]
struct RawCodeError<'a> {
    code: Cow<'a, str>,
    name: Cow<'a, str>,
}

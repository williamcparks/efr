use std::borrow::Cow;

use efr_macros::CodeList;
use quick_xml::NsReader;
use serde::{Deserialize, Serialize};

use crate::codes_service::{CodeCaseCategoryAvailability, EfrCodesError, utils::CodeRow};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeCaseType<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub casecategory: Cow<'a, str>,

    pub initial: bool,
    pub fee: f64,
    pub willfileddate: CodeCaseCategoryAvailability,
    pub casestreetaddress: CodeCaseCategoryAvailability,
}

impl<'a> CodeRow<'a> for CodeCaseType<'a> {
    fn code_row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError> {
        let raw = match RawCodeCaseType::code_row(ns_reader) {
            Ok(Some(some)) => some,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };
        let fee = match raw.fee.parse() {
            Ok(ok) => ok,
            Err(err) => {
                return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                    "Failed To Parse `fee`: `{err}` Got: `{}`",
                    raw.fee
                ))));
            }
        };

        Ok(Some(Self {
            code: raw.code,
            name: raw.name,
            casecategory: raw.casecategory,

            initial: raw.initial.eq_ignore_ascii_case("true"),
            fee,

            willfileddate: CodeCaseCategoryAvailability::try_new(raw.willfileddate.as_ref())?,
            casestreetaddress: CodeCaseCategoryAvailability::try_new(
                raw.casestreetaddress.as_ref(),
            )?,
        }))
    }
}

#[derive(CodeList)]
#[codelist("Code Case Category")]
struct RawCodeCaseType<'a> {
    code: Cow<'a, str>,
    name: Cow<'a, str>,
    casecategory: Cow<'a, str>,

    initial: Cow<'a, str>,
    fee: Cow<'a, str>,
    willfileddate: Cow<'a, str>,
    casestreetaddress: Cow<'a, str>,
}

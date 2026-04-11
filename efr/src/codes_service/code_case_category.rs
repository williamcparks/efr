use std::borrow::Cow;

use efr_macros::CodeList;
use quick_xml::NsReader;
use serde::{Deserialize, Serialize};

use crate::codes_service::{EfrCodesError, utils::CodeRow};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeCaseCategory<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub ecfcasetype: Cow<'a, str>,

    pub procedureremedyinitial: CodeCaseCategoryAvailability,
    pub procedureremedysubsequent: CodeCaseCategoryAvailability,
    pub damageamountinitial: CodeCaseCategoryAvailability,
    pub damageamountsubsequent: CodeCaseCategoryAvailability,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum CodeCaseCategoryAvailability {
    NotAvailable,
    Available,
    Required,
}

impl<'a> CodeRow<'a> for CodeCaseCategory<'a> {
    fn code_row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError> {
        let raw = match RawCodeCaseCategory::code_row(ns_reader) {
            Ok(Some(some)) => some,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        Ok(Some(Self {
            code: raw.code,
            name: raw.name,
            ecfcasetype: raw.ecfcasetype,

            procedureremedyinitial: CodeCaseCategoryAvailability::try_new(
                raw.procedureremedyinitial.as_ref(),
            )?,
            procedureremedysubsequent: CodeCaseCategoryAvailability::try_new(
                raw.procedureremedysubsequent.as_ref(),
            )?,
            damageamountinitial: CodeCaseCategoryAvailability::try_new(
                raw.damageamountinitial.as_ref(),
            )?,
            damageamountsubsequent: CodeCaseCategoryAvailability::try_new(
                raw.damageamountsubsequent.as_ref(),
            )?,
        }))
    }
}

impl CodeCaseCategoryAvailability {
    pub(super) fn try_new(v: &str) -> Result<Self, EfrCodesError> {
        match v {
            "Not Available" => Ok(Self::NotAvailable),
            "Available" => Ok(Self::Available),
            "Required" => Ok(Self::Required),
            _ => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                "Unknown Case Category Availability: `{v}`"
            )))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, CodeList)]
#[codelist("Code Case Category")]
struct RawCodeCaseCategory<'a> {
    code: Cow<'a, str>,
    name: Cow<'a, str>,
    ecfcasetype: Cow<'a, str>,
    procedureremedyinitial: Cow<'a, str>,
    procedureremedysubsequent: Cow<'a, str>,
    damageamountinitial: Cow<'a, str>,
    damageamountsubsequent: Cow<'a, str>,
}

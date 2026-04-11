use std::borrow::Cow;

use efr_macros::CodeList;
use quick_xml::NsReader;
use serde::{Deserialize, Serialize};

use crate::codes_service::CodeCaseCategoryAvailability;
use crate::codes_service::{EfrCodesError, utils::CodeRow};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeFiling<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,

    pub fee: f64,
    pub casecategory: Cow<'a, str>,
    pub casetypeid: Cow<'a, str>,
    pub filingtype: CodeFilingType,
    pub iscourtuseonly: bool,
    pub civilclaimamount: CodeCaseCategoryAvailability,
    pub probateestateamount: CodeCaseCategoryAvailability,
    pub amountincontroversy: CodeCaseCategoryAvailability,
    pub useduedate: bool,
    pub isproposedorder: bool,
    pub iswaiverrequest: bool,
    pub excludecertificateofservice: bool,
    pub isserviceofprocess: bool,
    pub efspcode: Cow<'a, str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum CodeFilingType {
    Initial,
    Subsequent,
    Both,
    None,
}

impl<'a> CodeRow<'a> for CodeFiling<'a> {
    fn code_row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError> {
        let raw = match RawCodeFiling::code_row(ns_reader) {
            Ok(Some(some)) => some,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        let fee = match raw.fee.is_empty() {
            true => 0.0,
            false => match raw.fee.parse() {
                Ok(ok) => ok,
                Err(err) => {
                    return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Failed To Parse `fee`: `{err}` Got: `{}`",
                        raw.fee
                    ))));
                }
            },
        };

        Ok(Some(Self {
            code: raw.code,
            name: raw.name,

            fee,
            casecategory: raw.casecategory,
            casetypeid: raw.casetypeid,
            filingtype: CodeFilingType::try_new(raw.filingtype.as_ref())?,
            iscourtuseonly: raw.iscourtuseonly.eq_ignore_ascii_case("true"),
            civilclaimamount: CodeCaseCategoryAvailability::try_new(raw.civilclaimamount.as_ref())?,
            probateestateamount: CodeCaseCategoryAvailability::try_new(
                raw.probateestateamount.as_ref(),
            )?,
            amountincontroversy: CodeCaseCategoryAvailability::try_new(
                raw.amountincontroversy.as_ref(),
            )?,
            useduedate: raw.useduedate.eq_ignore_ascii_case("true"),
            isproposedorder: raw.isproposedorder.eq_ignore_ascii_case("true"),
            iswaiverrequest: raw.iswaiverrequest.eq_ignore_ascii_case("true"),
            excludecertificateofservice: raw
                .excludecertificateofservice
                .eq_ignore_ascii_case("true"),
            isserviceofprocess: raw.isserviceofprocess.eq_ignore_ascii_case("true"),
            efspcode: raw.efspcode,
        }))
    }
}

impl CodeFilingType {
    fn try_new(v: &str) -> Result<Self, EfrCodesError> {
        match v {
            "Initial" => Ok(Self::Initial),
            "Subsequent" => Ok(Self::Subsequent),
            "Both" => Ok(Self::Both),
            "None" => Ok(Self::None),
            _ => Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                "Failed To Parse `CodeFilingType` Expected One Of `Initial`, `Subsequent`, `Both`, `None`, Got: `{v}`"
            )))),
        }
    }
}

#[derive(CodeList)]
#[codelist("Code Filing")]
struct RawCodeFiling<'a> {
    code: Cow<'a, str>,
    #[codelist(optional)]
    name: Cow<'a, str>,
    #[codelist(optional)]
    fee: Cow<'a, str>,
    #[codelist(optional)]
    casecategory: Cow<'a, str>,
    #[codelist(optional)]
    casetypeid: Cow<'a, str>,
    #[codelist(optional)]
    filingtype: Cow<'a, str>,
    #[codelist(optional)]
    iscourtuseonly: Cow<'a, str>,
    #[codelist(optional)]
    civilclaimamount: Cow<'a, str>,
    #[codelist(optional)]
    probateestateamount: Cow<'a, str>,
    #[codelist(optional)]
    amountincontroversy: Cow<'a, str>,
    #[codelist(optional)]
    useduedate: Cow<'a, str>,
    #[codelist(optional)]
    isproposedorder: Cow<'a, str>,
    #[codelist(optional)]
    excludecertificateofservice: Cow<'a, str>,
    #[codelist(optional)]
    iswaiverrequest: Cow<'a, str>,
    #[codelist(optional)]
    isserviceofprocess: Cow<'a, str>,
    #[codelist(optional)]
    efspcode: Cow<'a, str>,
}

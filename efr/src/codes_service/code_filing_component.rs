use std::borrow::Cow;

use efr_macros::CodeList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CodeList, Deserialize, Serialize)]
#[codelist("Code Filing Component")]
pub struct CodeFilingComponent<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub filingcodeid: Cow<'a, str>,
    pub required: Cow<'a, str>,
    pub displayorder: Cow<'a, str>,
    pub allowmultiple: Cow<'a, str>,
    pub allowedfiletypes: Cow<'a, str>,
}

use std::borrow::Cow;

use efr_macros::CodeList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CodeList, Deserialize, Serialize)]
#[codelist("Code Filer Type")]
pub struct CodeFilerType<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub default: Cow<'a, str>,
}

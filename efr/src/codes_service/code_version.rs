use std::borrow::Cow;

use efr_macros::CodeList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, CodeList)]
#[codelist("Code Version")]
pub struct CodeVersion<'a> {
    pub location: Cow<'a, str>,
    pub codelist: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

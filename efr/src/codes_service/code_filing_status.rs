use std::borrow::Cow;

use efr_macros::CodeList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, CodeList)]
#[codelist("Code Filing Status")]
pub struct CodeFilingStatus<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

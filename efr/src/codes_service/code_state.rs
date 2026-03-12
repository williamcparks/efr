use std::borrow::Cow;

use efr_macros::CodeList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, CodeList)]
#[codelist("Code State")]
pub struct CodeState<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub countrycode: Cow<'a, str>,
}

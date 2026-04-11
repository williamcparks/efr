use std::borrow::Cow;

use efr_macros::CodeList;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CodeList, Deserialize, Serialize)]
#[codelist("Code Document Type")]
pub struct CodeDocumentType<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
    #[codelist(optional)]
    pub filingcodeid: Cow<'a, str>,
    pub iscourtuseonly: Cow<'a, str>,
    pub isdefault: Cow<'a, str>,
}

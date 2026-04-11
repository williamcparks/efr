use std::borrow::Cow;

use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct RawRow<'a> {
    #[serde(rename = "Value", default)]
    pub(super) values: Vec<RawRowValue<'a>>,
}

#[derive(Debug, Deserialize)]
pub(super) struct RawRowValue<'a> {
    #[serde(rename = "@ColumnRef")]
    pub(super) column_ref: Cow<'a, str>,

    #[serde(rename = "SimpleValue")]
    pub(super) simple_value: Cow<'a, str>,
}

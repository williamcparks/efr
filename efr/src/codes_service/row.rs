use std::{borrow::Cow, collections::HashMap};

use quick_xml::Reader;
use serde_json::{Map, Value};

use crate::codes_service::{
    EfrCodesError, column_set::ColumnUse, extract::extract, raw_row::RawRow,
};

pub(super) struct Row {
    pub(super) value: Value,
}

impl Row {
    pub fn try_new(
        columns: &HashMap<Cow<'_, str>, ColumnUse>,
        estimated_obj_size: usize,
        xml: &str,
        reader: &mut Reader<&'_ [u8]>,
    ) -> Result<Option<Self>, EfrCodesError> {
        let row_xml = match extract("Row", xml, reader)? {
            Some(some) => some,
            None => return Ok(None),
        };
        let raw_row: RawRow<'_> = quick_xml::de::from_str(row_xml)?;

        let mut map = Map::with_capacity(estimated_obj_size);

        for value in raw_row.values {
            map.insert(
                value.column_ref.into_owned(),
                Value::String(value.simple_value.into_owned()),
            );
        }
        for (col, is_required) in columns.iter() {
            if !map.contains_key(col.as_ref()) && is_required.is_required() {
                let key = match col {
                    Cow::Borrowed(brw) => (*brw).to_owned(),
                    Cow::Owned(own) => own.clone(),
                };
                map.insert(key, Value::Null);
            }
        }

        Ok(Some(Self {
            value: Value::Object(map),
        }))
    }
}

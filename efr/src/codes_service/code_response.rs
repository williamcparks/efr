use std::{borrow::Cow, collections::HashMap};

use quick_xml::Reader;
use serde_json::Value;

use crate::codes_service::{
    EfrCodesError,
    column_set::{ColumnSet, ColumnUse},
    row::Row,
};

pub struct CodeResponse<'a> {
    xml: &'a str,
    reader: Reader<&'a [u8]>,
    columns: HashMap<Cow<'a, str>, ColumnUse>,
    estimated_obj_size: usize,
}

impl<'a> CodeResponse<'a> {
    pub fn try_new(xml: &'a str) -> Result<Self, EfrCodesError> {
        let mut reader = Reader::from_str(xml);
        let column_set = ColumnSet::try_new(xml, &mut reader)?;

        let estimated_obj_size = column_set.columns.len();
        let columns = column_set
            .columns
            .into_iter()
            .map(|col| (col.id, col.use_))
            .collect();

        Ok(Self {
            xml,
            reader,
            columns,
            estimated_obj_size,
        })
    }
}

impl<'a> Iterator for CodeResponse<'a> {
    type Item = Result<Value, EfrCodesError>;

    fn next(&mut self) -> Option<Self::Item> {
        match Row::try_new(
            &self.columns,
            self.estimated_obj_size,
            self.xml,
            &mut self.reader,
        ) {
            Ok(None) => None,
            Ok(Some(some)) => Some(Ok(some.value)),
            Err(err) => Some(Err(err)),
        }
    }
}

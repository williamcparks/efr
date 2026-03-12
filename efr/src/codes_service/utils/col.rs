use std::borrow::Cow;

use quick_xml::events::attributes::Attributes;

use crate::codes_service::EfrCodesError;

use super::{Pull, StartOrEmpty, Tag};

pub struct Col<'a> {
    pub column_ref: String,
    pub value: Cow<'a, str>,
}

impl<'a, 'b> Pull<'a, 'b> {
    fn extract_column_ref<'c>(attrs: Attributes<'c>) -> Result<Cow<'c, str>, EfrCodesError> {
        for attr_result in attrs {
            let attr = attr_result?;

            if attr.key.0 != b"ColumnRef" {
                continue;
            }

            return Ok(attr.unescape_value()?);
        }

        Err(EfrCodesError::Xml(quick_xml::DeError::Custom(
            "Expected <Value> To Have Attribute `ColumnRef` In Genericode XML".to_owned(),
        )))
    }

    pub fn col(&mut self) -> Result<Col<'a>, EfrCodesError> {
        let value = match self.start_or_empty(Tag::local("Value"))? {
            StartOrEmpty::Start(start) => start,
            StartOrEmpty::Empty(empty) => {
                let column_ref = Self::extract_column_ref(empty.attributes())?.into_owned();

                return Ok(Col {
                    column_ref,
                    value: Cow::Borrowed(""),
                });
            }
        };

        let column_ref = Self::extract_column_ref(value.attributes())?.into_owned();

        let simple_value = match self.start_or_empty(Tag::local("SimpleValue"))? {
            StartOrEmpty::Start(start) => start,
            StartOrEmpty::Empty(_) => {
                self.0.read_to_end(value.name())?;

                return Ok(Col {
                    column_ref,
                    value: Cow::Borrowed(""),
                });
            }
        };

        let column_value = self.0.read_text(simple_value.name())?;

        self.0.read_to_end(value.name())?;

        Ok(Col {
            column_ref,
            value: column_value,
        })
    }
}

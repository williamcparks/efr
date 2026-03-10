use std::borrow::Cow;

use quick_xml::events::attributes::Attributes;

use crate::codes_service::{
    EfrCodesError,
    pull::{Pull, Tag},
};

impl<'a, 'b> Pull<'a, 'b> {
    pub fn is_col(attrs: Attributes<'_>, column_name: &str) -> Result<bool, EfrCodesError> {
        for attr_result in attrs {
            let attr = attr_result?;

            if attr.key.0 != b"ColumnRef" {
                continue;
            }

            return Ok(attr.value.as_ref() == column_name.as_bytes());
        }

        Ok(false)
    }

    pub fn col(&mut self, column_name: &str) -> Result<Cow<'a, str>, EfrCodesError> {
        let value = self.open(Tag::local("Value"))?;

        if !Self::is_col(value.attributes(), column_name)? {
            return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                "Expected Column `{column_name}`"
            ))));
        }

        let simple_value = self.open(Tag::local("SimpleValue"))?;
        let res = self.0.read_text(simple_value.name())?;

        self.0.read_to_end(value.name())?;

        Ok(res)
    }
}

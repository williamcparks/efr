use quick_xml::NsReader;

use crate::codes_service::EfrCodesError;

use super::{CodeMetadata, CodeRows, code_rows::CodeRow};

#[derive(Clone, Debug)]
pub struct CodeResponse<'a, T> {
    pub codes_metadata: CodeMetadata<'a>,
    pub rows: CodeRows<'a, T>,
}

impl<'a, T: CodeRow<'a>> CodeResponse<'a, T> {
    pub fn try_new(xml: &'a str) -> Result<Self, EfrCodesError> {
        let mut ns_reader = NsReader::from_str(xml);
        ns_reader.config_mut().trim_text(true);

        let codes_metadata = CodeMetadata::try_new(&mut ns_reader)?;

        Ok(Self {
            codes_metadata,
            rows: CodeRows::new(ns_reader),
        })
    }
}

use quick_xml::NsReader;

use crate::codes_service::{CodesMetadata, EfrCodesError, Rows, rows::Row};

#[derive(Clone, Debug)]
pub struct CodesResponse<'a, T> {
    pub codes_metadata: CodesMetadata<'a>,
    pub rows: Rows<'a, T>,
}

impl<'a, T: Row<'a>> CodesResponse<'a, T> {
    pub fn try_new(xml: &'a str) -> Result<Self, EfrCodesError> {
        let mut ns_reader = NsReader::from_str(xml);
        ns_reader.config_mut().trim_text(true);

        let codes_metadata = CodesMetadata::try_new(&mut ns_reader)?;

        Ok(Self {
            codes_metadata,
            rows: Rows::new(ns_reader),
        })
    }
}

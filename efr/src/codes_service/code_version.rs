use std::borrow::Cow;

use quick_xml::name::QName;

use crate::codes_service::{pull::Pull, rows::Row};

#[derive(Clone, Debug)]
pub struct CodeVersion<'a> {
    pub location: Cow<'a, str>,
    pub codelist: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

impl<'a> Row<'a> for CodeVersion<'a> {
    fn row(
        ns_reader: &mut quick_xml::NsReader<&'a [u8]>,
    ) -> Result<Option<Self>, super::EfrCodesError> {
        let mut pull = Pull(ns_reader);
        if !pull.open_row()? {
            return Ok(None);
        }

        let location = pull.col("location")?;
        let codelist = pull.col("codelist")?;
        let version = pull.col("version")?;

        pull.0.read_to_end(QName(b"Row"))?;

        Ok(Some(Self {
            location,
            codelist,
            version,
        }))
    }
}

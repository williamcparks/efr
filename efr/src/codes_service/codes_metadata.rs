use std::borrow::Cow;

use quick_xml::NsReader;

use crate::codes_service::EfrCodesError;

use super::pull::{Pull, Tag};

#[derive(Clone, Debug)]
pub struct CodesMetadata<'a> {
    pub short_name: Cow<'a, str>,
    pub long_name: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub canonical_uri: Cow<'a, str>,
    pub canonical_version_uri: Cow<'a, str>,
}

impl<'a> CodesMetadata<'a> {
    pub fn try_new(reader: &mut NsReader<&'a [u8]>) -> Result<Self, EfrCodesError> {
        let mut pull = Pull(reader);

        pull.xml_decl()?;
        pull.open(Tag::new(
            "CodeList",
            "http://docs.oasis-open.org/codelist/ns/genericode/1.0/",
        ))?;

        let identification = pull.open(Tag::local("Identification"))?;
        let short_name = pull.open(Tag::local("ShortName"))?;
        let short_name = pull.0.read_text(short_name.name())?;

        let long_name = pull.open(Tag::local("LongName"))?;
        let long_name = pull.0.read_text(long_name.name())?;

        let version = pull.open(Tag::local("Version"))?;
        let version = pull.0.read_text(version.name())?;

        let canonical_uri = pull.open(Tag::local("CanonicalUri"))?;
        let canonical_uri = pull.0.read_text(canonical_uri.name())?;

        let canonical_version_uri = pull.open(Tag::local("CanonicalVersionUri"))?;
        let canonical_version_uri = pull.0.read_text(canonical_version_uri.name())?;

        pull.0.read_to_end(identification.name())?;

        let col_set = pull.open(Tag::local("ColumnSet"))?;
        pull.0.read_to_end(col_set.name())?;

        pull.open(Tag::local("SimpleCodeList"))?;

        Ok(Self {
            short_name,
            long_name,
            version,
            canonical_uri,
            canonical_version_uri,
        })
    }
}

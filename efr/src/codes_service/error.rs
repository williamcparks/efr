use thiserror::Error;

#[derive(Debug, Error)]
pub enum EfrCodesHeaderError {
    /// a der parsing / serialization error
    #[error("EFile Rust Der (Crypto): {0}")]
    Der(
        #[from]
        #[source]
        der::Error,
    ),
}

#[derive(Debug, Error)]
pub enum EfrCodesError {
    #[error("EFile Rust Zip: {0}")]
    Zip(
        #[from]
        #[source]
        zip::result::ZipError,
    ),

    #[error("EFile Rust Zip: No Xml Codes File Insize Zip File")]
    NoXmlCodesFileInsideZip,

    #[error("EFile Rust XML: {0}")]
    Xml(
        #[from]
        #[source]
        quick_xml::DeError,
    ),
}

impl From<quick_xml::Error> for EfrCodesError {
    fn from(value: quick_xml::Error) -> Self {
        Self::Xml(quick_xml::DeError::InvalidXml(value))
    }
}

impl From<quick_xml::encoding::EncodingError> for EfrCodesError {
    fn from(value: quick_xml::encoding::EncodingError) -> Self {
        Self::Xml(quick_xml::DeError::InvalidXml(value.into()))
    }
}

impl From<quick_xml::events::attributes::AttrError> for EfrCodesError {
    fn from(value: quick_xml::events::attributes::AttrError) -> Self {
        Self::Xml(quick_xml::DeError::InvalidXml(value.into()))
    }
}

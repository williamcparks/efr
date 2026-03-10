use thiserror::Error;

#[derive(Debug, Error)]
pub enum EfrError {
    #[error("EFile Rust Invalid Multipart Response: {0}")]
    InvalidMultiPartResponse(Box<str>),

    #[error("EFile Rust XML: {0}")]
    Xml(
        #[from]
        #[source]
        quick_xml::DeError,
    ),

    #[error("EFile Rust EFM: {error_code} - {message}")]
    Efm { error_code: i64, message: Box<str> },
}

impl From<quick_xml::Error> for EfrError {
    fn from(value: quick_xml::Error) -> Self {
        Self::Xml(quick_xml::DeError::InvalidXml(value))
    }
}

impl From<quick_xml::events::attributes::AttrError> for EfrError {
    fn from(value: quick_xml::events::attributes::AttrError) -> Self {
        Self::Xml(quick_xml::DeError::InvalidXml(
            quick_xml::Error::InvalidAttr(value),
        ))
    }
}

impl From<quick_xml::encoding::EncodingError> for EfrError {
    fn from(value: quick_xml::encoding::EncodingError) -> Self {
        Self::Xml(quick_xml::DeError::InvalidXml(quick_xml::Error::Encoding(
            value,
        )))
    }
}

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

    #[error("EFile Rust XML To JSON: {0}")]
    XmlToJson(
        #[from]
        #[source]
        quick_xml_to_json::XmlToJsonError,
    ),

    #[error("EFile Rust EFM: {error_code} - {message}")]
    Efm { error_code: i64, message: Box<str> },
}

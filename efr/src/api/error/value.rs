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

    #[error("EFile Rust: {code} / {message}")]
    Api { code: i64, message: Box<str> },

    #[error("EFile Rust SOAP Fault: {code} / {message}")]
    Fault { code: Box<str>, message: Box<str> },
}

impl From<quick_xml::Error> for EfrError {
    fn from(value: quick_xml::Error) -> Self {
        Self::Xml(quick_xml::DeError::InvalidXml(value))
    }
}

impl EfrError {
    pub(crate) fn api(xml: &str) -> Option<Self> {
        use super::{api::ApiError, fault::FaultError};

        if let Some(api) = ApiError::new_opt(xml) {
            Some(Self::Api {
                code: api.code,
                message: api.message,
            })
        } else {
            FaultError::new_opt(xml).map(|fault| Self::Fault {
                code: fault.code,
                message: fault.message,
            })
        }
    }
}

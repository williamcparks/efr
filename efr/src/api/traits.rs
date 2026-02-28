use rsa::pkcs1v15::SigningKey;
use sha1::Sha1;

use crate::api::{EfrError, MultiPartRequest};

pub trait EfrRequest {
    const SOAP_ACTION: &'static str;
    const SOAP_ACTION_HEADER_NAME: &'static str = "SOAPAction";

    fn efr_request(&self, signing_key: &SigningKey<Sha1>, cert_der: &[u8]) -> MultiPartRequest;
}

pub trait EfrResponse<'a>: Sized {
    fn efr_response(response: &'a str) -> Result<Self, EfrError>;
}

pub(crate) trait Xml {
    fn xml(&self, xml: &mut Vec<u8>);

    fn len(&self) -> usize;
}

impl Xml for str {
    fn xml(&self, xml: &mut Vec<u8>) {
        xml.extend_from_slice(self.as_bytes())
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Xml> Xml for Option<T> {
    fn xml(&self, xml: &mut Vec<u8>) {
        if let Some(inner) = self.as_ref() {
            T::xml(inner, xml)
        }
    }

    fn len(&self) -> usize {
        match self.as_ref() {
            Some(inner) => T::len(inner),
            None => 0,
        }
    }
}

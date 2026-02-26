use efr_macros::xml;
use rsa::pkcs1v15::SigningKey;
use serde::{Deserialize, Serialize};
use sha1::Sha1;

use crate::api::{EfrRequest, MultiPartRequest, insecure_request};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResetPasswordRequest<'a> {
    pub email: &'a str,
}

impl<'a> EfrRequest for ResetPasswordRequest<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmUserService/ResetPassword";

    fn efr_request(&self, signing_key: &SigningKey<Sha1>, cert_der: &[u8]) -> MultiPartRequest {
        insecure_request(self, signing_key, cert_der)
    }
}

xml! {
    impl<'a> Xml for ResetPasswordRequest<'a> {
        @ns {
            reset = "urn:tyler:efm:services:schema:ResetPasswordRequest";
        }

        reset:ResetPasswordRequest {
            reset:Email { (self.email) }
        }
    }
}

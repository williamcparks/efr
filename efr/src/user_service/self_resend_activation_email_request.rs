use efr_macros::xml;
use rsa::pkcs1v15::SigningKey;
use serde::{Deserialize, Serialize};
use sha1::Sha1;

use crate::api::{EfrRequest, MultiPartRequest, insecure_request};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SelfResendActivationEmailRequest<'a> {
    pub email: &'a str,
}

impl<'a> EfrRequest for SelfResendActivationEmailRequest<'a> {
    const SOAP_ACTION: &'static str =
        "urn:tyler:efm:services/IEfmUserService/SelfResendActivationEmail";

    fn efr_request(&self, signing_key: &SigningKey<Sha1>, cert_der: &[u8]) -> MultiPartRequest {
        insecure_request(self, signing_key, cert_der)
    }
}

xml! {
    impl<'a> Xml for SelfResendActivationEmailRequest<'a> {
        @ns {
            resend = "urn:tyler:efm:services:schema:ResendActivationEmailRequest";
        }

        resend:SelfResendActivationEmailRequest {
            resend:Email { (self.email) }
        }
    }
}

use efr_macros::xml;
use rsa::{pkcs1v15::SigningKey, traits::PublicKeyParts};
use serde::{Deserialize, Serialize};
use sha1::Sha1;

use crate::api::{
    EfrRequest, Envelope, MultiPartRequest, MultiPartRequestBuilder, SecurityHeader, Xml,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthenticateUserRequest<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

impl<'a> EfrRequest for AuthenticateUserRequest<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmUserService/AuthenticateUser";

    fn efr_request(&self, signing_key: &SigningKey<Sha1>, cert_der: &[u8]) -> MultiPartRequest {
        let len = MultiPartRequestBuilder::MUTLI_PART_REQUEST_OVERHEAD
            + Envelope::ENVELOPE_OVERHEAD
            + SecurityHeader::SECURITY_HEADER_OVERHEAD
            + signing_key.as_ref().size().div_ceil(3) * 4
            + cert_der.len()
            + self.len();

        let mut builder = MultiPartRequestBuilder::new(len);
        let sec = SecurityHeader::new(signing_key, builder.cert_content_uuid);

        let xml = builder.xml();
        xml.extend_from_slice(Envelope::START_HEADER);
        sec.xml(xml);
        xml.extend_from_slice(Envelope::END_HEADER_START_BODY);
        self.xml(xml);
        xml.extend_from_slice(Envelope::END_BODY);

        builder.cert_der(cert_der)
    }
}

xml! {
    impl<'a> Xml for AuthenticateUserRequest<'a> {
        @ns {
            tyler = "urn:tyler:efm:services";
            auth = "urn:tyler:efm:services:schema:AuthenticateRequest";
        }

        tyler:AuthenticateUser {
            tyler:AuthenticateRequest {
                auth:Email { (self.email) }
                auth:Password { (self.password) }
            }
        }
    }
}

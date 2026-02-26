use efr_macros::xml;
use rsa::pkcs1v15::SigningKey;
use serde::{Deserialize, Serialize};
use sha1::Sha1;

use crate::api::{EfrRequest, MultiPartRequest, insecure_request};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthenticateUserRequest<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

impl<'a> EfrRequest for AuthenticateUserRequest<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmUserService/AuthenticateUser";

    fn efr_request(&self, signing_key: &SigningKey<Sha1>, cert_der: &[u8]) -> MultiPartRequest {
        insecure_request(self, signing_key, cert_der)
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

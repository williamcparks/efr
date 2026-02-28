use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangePasswordRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub old_password: &'a str,
    pub new_password: &'a str,
}

impl<'a> SecureEfrRequest for ChangePasswordRequest<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmUserService/ChangePassword";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for ChangePasswordRequest<'a> {
        @ns {
            change = "urn:tyler:efm:services:schema:ChangePasswordRequest";
            tyler = "urn:tyler:efm:services";
        }

        tyler:ChangePassword {
            tyler:ChangePasswordRequest {
                change:OldPassword { (self.old_password) }
                change:NewPassword { (self.new_password) }
                change:PasswordQuestion {}
                change:PasswordAnswer {}
            }
        }
    }
}

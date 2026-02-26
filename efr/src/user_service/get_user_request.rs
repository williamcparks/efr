use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetUserRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub user_id: &'a str,
}

impl<'a> SecureEfrRequest for GetUserRequest<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmUserService/GetUser";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for GetUserRequest<'a> {
        @ns {
            user = "urn:tyler:efm:services:schema:GetUserRequest";
        }

        user:GetUserRequest {
            user:UserID { (self.user_id) }
        }
    }
}

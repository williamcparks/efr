use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateUserRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub user_id: &'a str,
    pub new_email: &'a str,
    pub first_name: &'a str,
    pub middle_name: &'a str,
    pub last_name: &'a str,
}

impl<'a> SecureEfrRequest for UpdateUserRequest<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmUserService/UpdateUser";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for UpdateUserRequest<'a> {
        @ns {
            tyler = "urn:tyler:efm:services";
            common="urn:tyler:efm:services:schema:Common";
        }

        tyler:UpdateUser {
            tyler:UpdateUserRequest {
                common:User UserID=(self.user_id) {
                    common:Email { (self.new_email) }
                    common:FirstName { (self.first_name) }
                    common:MiddleName { (self.middle_name) }
                    common:LastName { (self.last_name) }
                }
            }
        }
    }
}

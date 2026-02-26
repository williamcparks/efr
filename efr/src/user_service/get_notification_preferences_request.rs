use serde::{Deserialize, Serialize};

use crate::api::{SecureEfrRequest, Xml};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetNotificationPreferencesRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
}

impl<'a> SecureEfrRequest for GetNotificationPreferencesRequest<'a> {
    const SOAP_ACTION: &'static str =
        "urn:tyler:efm:services/IEfmUserService/GetNotificationPreferences";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

impl<'a> Xml for GetNotificationPreferencesRequest<'a> {
    fn len(&self) -> usize {
        0
    }

    fn xml(&self, _xml: &mut Vec<u8>) {}
}

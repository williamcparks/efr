use serde::{Deserialize, Serialize};

use crate::api::{SecureEfrRequest, Xml};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetPaymentAccountTypeListRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
}

impl<'a> SecureEfrRequest for GetPaymentAccountTypeListRequest<'a> {
    const SOAP_ACTION: &'static str =
        "urn:tyler:efm:services/IEfmFirmService/GetPaymentAccountTypeList";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

impl<'a> Xml for GetPaymentAccountTypeListRequest<'a> {
    fn len(&self) -> usize {
        0
    }

    fn xml(&self, _xml: &mut Vec<u8>) {}
}

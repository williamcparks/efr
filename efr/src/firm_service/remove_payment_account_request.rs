use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemovePaymentAccountRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub payment_account_id: &'a str,
}

impl<'a> SecureEfrRequest for RemovePaymentAccountRequest<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmFirmService/RemovePaymentAccount";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for RemovePaymentAccountRequest<'a> {
        @ns {
            tyler = "urn:tyler:efm:services";
            remove="urn:tyler:efm:services:schema:RemovePaymentAccountRequest";
        }

        tyler:RemovePaymentAccount {
            tyler:RemovePaymentAccountRequest {
                remove:PaymentAccountID { (self.payment_account_id) }
            }
        }
    }
}

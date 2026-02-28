use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetPaymentAccountRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub payment_account_id: &'a str,
}

impl<'a> SecureEfrRequest for GetPaymentAccountRequest<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmFirmService/GetPaymentAccount";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for GetPaymentAccountRequest<'a> {
        @ns {
            payment = "urn:tyler:efm:services:schema:GetPaymentAccountRequest";
            tyler = "urn:tyler:efm:services";
        }

        tyler:GetPaymentAccount {
            tyler:GetPaymentAccountRequest {
                payment:PaymentAccountID { (self.payment_account_id) }
            }
        }
    }
}

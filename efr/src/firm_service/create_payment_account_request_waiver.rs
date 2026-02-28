use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentAccountRequestWaiver<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub account_name: &'a str,
    pub payment_account_type_code: &'a str,
    pub payment_account_type_code_id: &'a str,
}

impl<'a> SecureEfrRequest for CreatePaymentAccountRequestWaiver<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmFirmService/CreatePaymentAccount";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for CreatePaymentAccountRequestWaiver<'a> {
        @ns {
            tyler = "urn:tyler:efm:services";
            common = "urn:tyler:efm:services:schema:Common";
            xsi="http://www.w3.org/2001/XMLSchema-instance";
        }

        tyler:CreatePaymentAccount {
            tyler:CreatePaymentAccountRequest {
                common:PaymentAccount PaymentAccountTypeCode=(self.payment_account_type_code) {
                    common:AccountName { (self.account_name) }
                    common:PaymentAccountTypeCodeId { (self.payment_account_type_code_id) }
                    common:AccountToken xsi:nil="true" {}
                }
            }
        }
    }
}

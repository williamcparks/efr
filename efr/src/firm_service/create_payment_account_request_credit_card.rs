use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePaymentAccountRequestCreditCard<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub account_name: &'a str,
    pub payment_account_type_code: &'a str,
    pub payment_account_type_code_id: &'a str,
    pub account_token: &'a str,
    pub card_type: &'a str,
    pub card_last_4: &'a str,
    pub card_month: &'a str,
    pub card_year: &'a str,
    pub card_holder_name: &'a str,
}

impl<'a> SecureEfrRequest for CreatePaymentAccountRequestCreditCard<'a> {
    const SOAP_ACTION: &'static str = "urn:tyler:efm:services/IEfmFirmService/CreatePaymentAccount";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for CreatePaymentAccountRequestCreditCard<'a> {
        @ns {
            tyler = "urn:tyler:efm:services";
            common = "urn:tyler:efm:services:schema:Common";
        }

        tyler:CreatePaymentAccount {
            tyler:CreatePaymentAccountRequest {
                common:PaymentAccount PaymentAccountTypeCode=(self.payment_account_type_code) {
                    common:AccountName { (self.account_name) }
                    common:PaymentAccountTypeCodeId { (self.payment_account_type_code_id) }
                    common:AccountToken { (self.account_token) }
                    common:CardType { (self.card_type) }
                    common:CardLast4 { (self.card_last_4) }
                    common:CardMonth { (self.card_month) }
                    common:CardYear { (self.card_year) }
                    common:CardHolderName { (self.card_holder_name) }
                }
            }
        }
    }
}

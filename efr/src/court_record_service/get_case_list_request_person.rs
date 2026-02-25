use chrono::Utc;
use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetCaseListRequestPerson<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
    pub efsp_url: &'a str,
    pub jurisdiction: &'a str,
    pub first_name: &'a str,
    pub middle_name: &'a str,
    pub last_name: &'a str,
}

impl<'a> SecureEfrRequest for GetCaseListRequestPerson<'a> {
    const SOAP_ACTION: &'static str =
        "https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0WSDL/CourtRecordMDE/GetCaseList";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for GetCaseListRequestPerson<'a> {
        @ns {
            nc="http://release.niem.gov/niem/niem-core/4.0/";
            j="http://release.niem.gov/niem/domains/jxdm/6.1/";
            ecf="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/ecf";
            caselistrequest="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/caselistrequest";
            wrappers = "https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/wrappers";
        }

        wrappers:GetCaseListRequest {
            caselistrequest:GetCaseListRequestMessage {
                nc:DocumentIdentification {
                    nc:IdentificationID { "1" }
                    nc:IdentificationCategoryDescriptionText { "messageID" }
                    nc:IdentificationSourceText { "FilingReview" }
                }

                ecf:SendingMDELocationID {
                    nc:IdentificationID { (self.efsp_url) }
                }

                ecf:ServiceInteractionProfileCode {
                    "urn:oasis:names:tc:legalxml-courtfiling:schema:xsd:WebServicesMessaging-5.0"
                }

                j:CaseCourt {
                    nc:OrganizationIdentification {
                        nc:IdentificationID { (self.jurisdiction) }
                    }
                }

                nc:DocumentPostDate {
                    nc:DateTime { (&Utc::now()) }
                }

                caselistrequest:CaseListQueryCriteria {
                    caselistrequest:CaseParticipant {
                        nc:EntityPerson {
                            nc:PersonName {
                                nc:PersonGivenName { (self.first_name) }
                                nc:PersonMiddleName { (self.middle_name) }
                                nc:PersonSurName { (self.last_name) }
                            }
                        }
                    }
                }
            }
        }
    }
}

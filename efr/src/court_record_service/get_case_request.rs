use chrono::Utc;
use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetCaseRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,
    pub efsp_url: &'a str,
    pub jurisdiction: &'a str,
    pub case_tracking_id: &'a str,
}

impl<'a> SecureEfrRequest for GetCaseRequest<'a> {
    const SOAP_ACTION: &'static str =
        "https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0WSDL/CourtRecordMDE/GetCase";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for GetCaseRequest<'a> {
        @ns {
            nc="http://release.niem.gov/niem/niem-core/4.0/";
            j="http://release.niem.gov/niem/domains/jxdm/6.1/";
            ecf="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/ecf";
            wrappers = "https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/wrappers";
            caserequest="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/caserequest";
            // common="urn:tyler:ecf:v5.0:extensions:common";
        }


        wrappers:GetCaseRequest {
            caserequest:GetCaseRequestMessage {
                nc:DocumentIdentification {
                    nc:IdentificationID { "1" }
                    nc:IdentificationCategoryDescriptionText { "messageID" }
                    nc:IdentificationSourceText { "FilingAssembly" }
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

                caserequest:CaseQueryCriteria {
                    caserequest:IncludeCalendarEventIndicator { "false" }
                    caserequest:IncludeDocketEntryIndicator { "false" }
                    caserequest:IncludeParticipantsIndicator { "true" }
                    ecf:CaseTrackingID {
                        nc:IdentificationID { (self.case_tracking_id) }
                    }
                    j:CaseNumberText {}
                }
            }
        }
    }
}

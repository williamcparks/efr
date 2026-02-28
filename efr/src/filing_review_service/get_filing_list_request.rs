use chrono::{DateTime, Utc};
use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetFilingListRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub efsp_url: &'a str,

    pub jurisdiction: Option<&'a str>,
    pub filing_status: Option<&'a str>,
    pub case_tracking_id: Option<&'a str>,
    pub case_number: Option<&'a str>,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub envelope_number: Option<&'a str>,
    pub submitter: Option<&'a str>,
}

impl<'a> SecureEfrRequest for GetFilingListRequest<'a> {
    const SOAP_ACTION: &'static str = "https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0WSDL/FilingReviewMDE/GetFilingList";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for GetFilingListRequest<'a> {
        @ns {
            wrappers="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/wrappers";
            filinglistrequest="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/filinglistrequest";
            nc = "http://release.niem.gov/niem/niem-core/4.0/";
            ecf="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/ecf";
        }

        wrappers:GetFilingListRequest {
            filinglistrequest:GetFilingListRequestMessage {
                nc:DocumentIdentification {
                    nc:IdentificationID { "1" }
                    nc:IdentificationCategoryDescriptionText { "messageID" }
                    nc:IdentificationSourceText { "FilingAssembly" }
                }

                ecf:SendingMDELocationID {
                    nc:IdentificationID {
                        (self.efsp_url)
                    }
                }

                ecf:ServiceInteractionProfileCode {
                    "urn:oasis:names:tc:legalxml-courtfiling:schema:xsd:WebServicesMessaging-5.0"
                }

                (&self.jurisdiction.map(JurisFilter))

                nc:DocumentPostDate {
                    nc:DateTime { (&Utc::now()) }
                }

                filinglistrequest:FilingListQueryCriteria {
                    (&self.filing_status.map(FilingStatusFilter))

                    (&self.case_tracking_id.map(CaseTrackingIDFilter))

                    (&self.case_number.map(CaseNumberFilter))

                    (&self.date_range.map(DateRangeFilter))

                    (&self.envelope_number.map(EnvelopeFilter))

                    (&self.submitter.map(SubmitterFilter))
                }
            }
        }
    }
}

struct JurisFilter<'a>(&'a str);

xml! {
    impl<'a> Xml for JurisFilter<'a> {
        @ns {
            #[subelement]
            j;
            #[subelement]
            nc;
        }

        j:CaseCourt {
            nc:OrganizationIdentification {
                nc:IdentificationID {
                    (self.0)
                }
            }
        }
    }
}

struct FilingStatusFilter<'a>(&'a str);

xml! {
    impl<'a> Xml for FilingStatusFilter<'a> {
        @ns {
            common="urn:tyler:ecf:v5.0:extensions:common";
            #[subelement]
            ecf;
        }

        common:FilingListQueryCriteriaAugmentation {
            ecf:FilingStatus {
                ecf:FilingReviewStatusCode {
                    (self.0)
                }
            }
        }
    }
}

struct CaseTrackingIDFilter<'a>(&'a str);

xml! {
    impl<'a> Xml for CaseTrackingIDFilter<'a> {
        @ns {
            #[subelement]
            ecf;
            #[subelement]
            nc;
        }

        ecf:CaseTrackingID {
            nc:IdentificationID { (self.0) }
        }
    }
}

struct CaseNumberFilter<'a>(&'a str);

xml! {
    impl<'a> Xml for CaseNumberFilter<'a> {
        @ns {
            #[subelement]
            j;
        }

        j:CaseNumberText { (self.0) }
    }
}

struct DateRangeFilter((DateTime<Utc>, DateTime<Utc>));

xml! {
    impl Xml for DateRangeFilter {
        @ns {
            #[subelement]
            nc;
        }

        nc:DateRange {
            nc:StartDate {
                nc:Date { (&self.0.0) }
            }
            nc:EndDate {
                nc:Date { (&self.0.1) }
            }
        }
    }
}

struct EnvelopeFilter<'a>(&'a str);

xml! {
    impl<'a> Xml for EnvelopeFilter<'a> {
        @ns {
            #[subelement]
            nc;
        }

        nc:DocumentIdentification {
            nc:IdentificationID { (self.0) }
        }
    }
}

struct SubmitterFilter<'a>(&'a str);

xml! {
    impl<'a> Xml for SubmitterFilter<'a> {
        @ns {
            #[subelement]
            nc;
        }

        nc:DocumentSubmitter {
            nc:EntityPerson {
                nc:PersonOtherIdentification {
                    nc:IdentificationID { (self.0) }
                }
            }
        }
    }
}

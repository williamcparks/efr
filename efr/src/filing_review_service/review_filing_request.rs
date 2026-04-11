use chrono::Utc;
use efr_macros::xml;
use serde::Serialize;

use crate::api::SecureEfrRequest;

#[derive(Clone, Debug, Serialize)]
pub struct ReviewFilingRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub efsp_url: &'a str,
    pub jurisdiction: &'a str,

    pub case_category_code: &'a str,
    pub case_type_code: &'a str,
    pub filing_code: &'a str,
    pub filing_component_code: &'a str,
    pub document_type_code: &'a str,
    pub motion_type_code: &'a str,
    pub filer_type_code: &'a str,

    pub document_description_text: &'a str,
    pub page_count: &'a str,
    pub original_file_name: &'a str,
    pub base_64: &'a str,
    pub binary_description_text: &'a str,
    pub binary_size_value: &'a str,
    pub filing_comments: &'a str,

    pub case_tracking_id: &'a str,

    pub payment_account_id: &'a str,
}

impl<'a> SecureEfrRequest for ReviewFilingRequest<'a> {
    const SOAP_ACTION: &'static str =
        "https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0WSDL/FilingReviewMDE/ReviewFiling";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for ReviewFilingRequest<'a> {
        @ns {
            wrappers="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/wrappers";
            filing="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/filing";
            nc="http://release.niem.gov/niem/niem-core/4.0/";
            ecf="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/ecf";
            j="http://release.niem.gov/niem/domains/jxdm/6.1/";
            payment="https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/payment";
            cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2";
            cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";

            #[parent]
            tyler="urn:tyler:ecf:v5.0:extensions:common";

            #[parent]
            structures="http://release.niem.gov/niem/structures/4.0/";
        }

        wrappers:ReviewFilingRequest {
            filing:FilingMessage {
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

                ecf:ServiceInteractionProfileCode { "urn:oasis:names:tc:legalxml-courtfiling:schema:xsd:WebServicesMessaging-5.0" }

                j:CaseCourt {
                    nc:OrganizationIdentification {
                        nc:IdentificationID {
                            (self.jurisdiction)
                        }
                    }
                }

                nc:DocumentPostDate {
                    nc:DateTime { (&Utc::now()) }
                }

                filing:FilingLeadDocument structures:id="Filing1" {
                    nc:DocumentDescriptionText { (self.document_description_text) }
                    nc:DocumentFiledDate {
                        nc:DateTime { (&Utc::now()) }
                    }
                    nc:DocumentIdentification {}
                    nc:DocumentSequenceID { "1" }
                    ecf:DocumentAugmentation {
                        ecf:DocumentRendition {
                            nc:DocumentIdentification {}
                            nc:Attachment structures:id="Document1" {
                                tyler:AttachmentAugmentation {
                                    tyler:FilingComponentCode { (self.filing_component_code) }
                                    tyler:PageCount { (self.page_count) }
                                    tyler:OriginalFileName { (self.original_file_name) }
                                    tyler:IsRedactedIndicator { "false" }
                                }
                                nc:Base64BinaryObject { (self.base_64) }
                                nc:BinaryDescriptionText { (self.binary_description_text) }
                                nc:BinarySizeValue { (self.binary_size_value) }
                            }
                        }
                        ecf:DocumentTypeCode { (self.document_type_code) }
                        ecf:RegisterActionDescriptionCode { (self.filing_code) }
                    }
                    tyler:DocumentAugmentation {
                        tyler:FilingAction { "EFile" }
                        tyler:FilingCommentsText { (self.filing_comments) }
                        tyler:MotionTypeCode { (self.motion_type_code) }
                    }
                }

                tyler:FilingMessageAugmentation {
                    tyler:FilerTypeCode {
                        (self.filer_type_code)
                    }
                }

                nc:Case {
                    j:CaseAugmentation {
                        j:CaseCourt {
                            nc:OrganizationIdentification {
                                nc:IdentificationID { (self.jurisdiction) }
                            }
                        }
                    }

                    ecf:CaseAugmentation {
                        ecf:CaseCategoryCode { (self.case_category_code) }
                        ecf:CaseNewIndicator { "false" }
                        ecf:CaseTrackingID {
                            nc:IdentificationID { (self.case_tracking_id) }
                        }

                        ecf:CaseTypeCode { (self.case_type_code) }
                    }
                }
            }

            payment:PaymentMessage {
                cac:Address {}
                cac:AllowanceCharge {
                    cbc:ChargeIndicator { "true" }
                    cbc:Amount currencyID="USD" { "0.00" }
                    cac:TaxCategory {
                        cbc:Percent { ".0000" }
                        cac:TaxScheme {}
                    }
                    cac:TaxTotal {
                        cbc:TaxAmount currencyID="USD" { ".00" }
                    }
                    cac:PaymentMeans {
                        cbc:PaymentMeansCode {}
                        cbc:PaymentID { (self.payment_account_id) }
                    }
                }
                cac:Payment {}
                payment:CorrectedPaymentIndicator { "false" }
                payment:Payer {}
            }
        }
    }
}

use serde::{Deserialize, Serialize, de::Visitor};

use crate::{
    api::{EfrError, EfrResponse, MultiPartResponse},
    court_record_service::common::OrganizationIdentification,
};

use super::common::JCaseAugmentation;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetCaseResponse<'a> {
    pub case_title_text: &'a str,
    pub jurisdiction: &'a str,
    pub case_category_code: &'a str,
    pub case_tracking_id: &'a str,
    pub case_type_code: &'a str,
    pub case_number_text: &'a str,
}

impl<'a> EfrResponse<'a> for GetCaseResponse<'a> {
    fn efr_response(response: &'a str) -> Result<Self, EfrError> {
        let multipart = MultiPartResponse::try_new(response)?;
        let xml = multipart.xml;

        if let Some(err) = EfrError::api(xml) {
            return Err(err);
        }

        let envelope: Envelope = quick_xml::de::from_str(xml)?;
        let detail = envelope
            .body
            .get_case_response
            .get_case_response_message
            .case;
        Ok(Self {
            case_title_text: detail.case_title_text,
            jurisdiction: detail
                .j_case_augmentation
                .case_court
                .organization_identification
                .identification_id,
            case_category_code: detail.ecf_case_augmentation.case_category_code,
            case_tracking_id: detail
                .ecf_case_augmentation
                .case_tracking_id
                .identification_id,
            case_type_code: detail.ecf_case_augmentation.case_type_code,
            case_number_text: detail.ecf_case_augmentation.case_number_text,
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(bound = "'a: 'de")]
struct Envelope<'a> {
    #[serde(borrow)]
    body: Body<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(bound = "'a: 'de")]
struct Body<'a> {
    #[serde(borrow)]
    get_case_response: GetCaseResponseRaw<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(bound = "'a: 'de")]
struct GetCaseResponseRaw<'a> {
    #[serde(borrow)]
    get_case_response_message: GetCaseResponseMessage<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(bound = "'a: 'de")]
struct GetCaseResponseMessage<'a> {
    #[serde(borrow)]
    case: CaseDetailRaw<'a>,
}

struct CaseDetailRaw<'a> {
    case_title_text: &'a str,
    j_case_augmentation: JCaseAugmentation<'a>,
    ecf_case_augmentation: EcfCaseAugmentation<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EcfCaseAugmentation<'a> {
    case_category_code: &'a str,
    #[serde(rename = "CaseParty")]
    _case_parties: Vec<()>,
    #[serde(rename = "CaseTrackingID")]
    case_tracking_id: OrganizationIdentification<'a>,
    case_type_code: &'a str,
    case_number_text: &'a str,
}

impl<'de> Deserialize<'de> for CaseDetailRaw<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["CaseTitleText"];

        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = CaseDetailRaw<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CaseDetailRaw")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut case_title_text: Option<&'de str> = None;
                let mut j_case_augmentation: Option<JCaseAugmentation<'de>> = None;
                let mut ecf_case_augmentation: Option<EcfCaseAugmentation<'de>> = None;

                while let Some(entry) = map.next_key::<&str>()? {
                    if entry == "CaseTitleText" {
                        case_title_text = Some(map.next_value()?);
                    } else if entry == "CaseAugmentation" {
                        if j_case_augmentation.is_none() {
                            j_case_augmentation = Some(map.next_value()?);
                        } else if ecf_case_augmentation.is_none() {
                            ecf_case_augmentation = Some(map.next_value()?);
                        }
                        /*
                        else {
                            return Err(serde::de::Error::duplicate_field("CaseAugmentation"));
                        }
                        */
                        else {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    } else {
                        return Err(serde::de::Error::unknown_field(entry, FIELDS));
                    }
                }

                let case_title_text = match case_title_text {
                    Some(some) => some,
                    None => return Err(serde::de::Error::missing_field("CaseTitleText")),
                };

                let j_case_augmentation = match j_case_augmentation {
                    Some(some) => some,
                    None => return Err(serde::de::Error::missing_field("j:CaseAugmentation")),
                };

                let ecf_case_augmentation = match ecf_case_augmentation {
                    Some(some) => some,
                    None => return Err(serde::de::Error::missing_field("ecf:CaseAugmentation")),
                };

                Ok(CaseDetailRaw {
                    case_title_text,
                    j_case_augmentation,
                    ecf_case_augmentation,
                })
            }
        }

        deserializer.deserialize_struct("CaseDetailRaw", FIELDS, Vis)
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(super) struct JCaseAugmentation<'a> {
    #[serde(borrow)]
    pub(super) case_court: CaseCourt<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(super) struct CaseCourt<'a> {
    #[serde(borrow)]
    pub(super) organization_identification: OrganizationIdentification<'a>,
}

#[derive(Deserialize)]
pub(super) struct OrganizationIdentification<'a> {
    #[serde(rename = "IdentificationID")]
    pub(super) identification_id: &'a str,
}

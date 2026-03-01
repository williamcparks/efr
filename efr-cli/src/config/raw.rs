use efr::api::Metadata;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawEfrConfig {
    pub metadata: Metadata,
    pub credentials: RawEfrConfigCredentials,
    pub admin: EfrConfigAdmin,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawEfrConfigCredentials {
    pub cert_der: String,
    pub pkey_pem: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EfrConfigAdmin {
    pub email: String,
    pub password: String,
}

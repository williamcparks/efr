use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::{EfrError, EfrResponse, MultiPartResponse};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthenticateUserResponse<'a> {
    pub user_id: &'a str,
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password_hash: &'a str,
    pub expiration: DateTime<Utc>,
}

impl<'a> EfrResponse<'a> for AuthenticateUserResponse<'a> {
    fn efr_response(response: &'a str) -> Result<Self, EfrError> {
        let multipart = MultiPartResponse::try_new(response)?;
        let xml = multipart.xml;

        if let Some(err) = EfrError::api(xml) {
            return Err(err);
        }

        let envelope: Envelope = quick_xml::de::from_str(xml)?;
        let inner = envelope
            .body
            .authenticate_user_response
            .authenticate_user_response;

        Ok(Self {
            user_id: inner.user_id,
            email: inner.email,
            first_name: inner.first_name,
            last_name: inner.last_name,
            password_hash: inner.password_hash,
            expiration: inner.expiration_date_time,
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Envelope<'a> {
    #[serde(borrow)]
    body: Body<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Body<'a> {
    #[serde(borrow)]
    authenticate_user_response: OuterResponse<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct OuterResponse<'a> {
    #[serde(borrow)]
    authenticate_user_response: Inner<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Inner<'a> {
    #[serde(rename = "UserID")]
    user_id: &'a str,
    email: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    password_hash: &'a str,
    #[serde(with = "crate::api::serde_datetime")]
    expiration_date_time: DateTime<Utc>,
}

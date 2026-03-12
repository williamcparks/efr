use efr::{
    api::{json, EfrRequest, Metadata},
    user_service::AuthenticateUserRequest,
};
use reqwest::{header::CONTENT_TYPE, Client};
use rsa::pkcs1v15::SigningKey;
use serde::Deserialize;
use serde_json::Value;

use crate::send::{sign, SendError};

#[derive(Deserialize)]
pub struct AuthenticateUser {
    pub email: Box<str>,
    pub password: Box<str>,
}

impl AuthenticateUser {
    pub async fn handler(&self, client: Client, metadata: Metadata) -> Result<Value, SendError> {
        let (private_key, cert_der) = sign()?;

        let request = AuthenticateUserRequest {
            email: self.email.as_ref(),
            password: self.password.as_ref(),
        }
        .efr_request(&SigningKey::new(private_key), cert_der);

        let xml = client
            .post(metadata.user_service_url())
            .header(CONTENT_TYPE, request.content_type())
            .header(
                AuthenticateUserRequest::SOAP_ACTION_HEADER_NAME,
                AuthenticateUserRequest::SOAP_ACTION,
            )
            .body(request.into_bytes())
            .send()
            .await?
            .text()
            .await?;

        Ok(json(xml.as_str())?)
    }
}

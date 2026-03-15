use efr::{
    api::{json, EfrRequest, Metadata},
    court_record_service::GetCaseListRequestCaseNumber,
};
use reqwest::{header::CONTENT_TYPE, Client};
use rsa::pkcs1v15::SigningKey;
use serde::Deserialize;
use serde_json::Value;

use crate::send::{sign, AuthToken, SendError};

#[derive(Deserialize)]
pub struct GetCaseList {
    pub case_number: Box<str>,
    pub jurisdiction: Box<str>,
}

impl GetCaseList {
    pub async fn handler(
        &self,
        client: Client,
        metadata: Metadata,
        authtoken: Option<AuthToken>,
    ) -> Result<Value, SendError> {
        let authtoken = authtoken.ok_or(SendError::AuthToken)?;
        let (email, password_hash) = authtoken.pair();

        let (private_key, cert_der) = sign()?;

        let request = GetCaseListRequestCaseNumber {
            email,
            password_hash,
            efsp_url: "https://efile.click",
            jurisdiction: self.jurisdiction.as_ref(),
            case_number: self.case_number.as_ref(),
        }
        .efr_request(&SigningKey::new(private_key), cert_der);

        println!(
            "{}",
            String::from_utf8_lossy(request.clone().into_bytes().as_slice())
        );

        let xml = client
            .post(metadata.court_record_service_url())
            .header(CONTENT_TYPE, request.content_type())
            .header(
                GetCaseListRequestCaseNumber::SOAP_ACTION_HEADER_NAME,
                GetCaseListRequestCaseNumber::SOAP_ACTION,
            )
            .body(request.into_bytes())
            .send()
            .await?
            .text()
            .await?;

        Ok(json(xml.as_str())?)
    }
}

use efr::{
    api::{EfrRequest, Environment, Metadata, State},
    user_service::AuthenticateUserRequest,
};
use lambda_http::{Body, Request, RequestPayloadExt, Response};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use rsa::{pkcs1v15::SigningKey, pkcs8::DecodePrivateKey, RsaPrivateKey};
use serde::Deserialize;
use thiserror::Error;

const PKEY_PEM: &str = include_str!("../../../env/pkey.pem");
const CERT_DER: &[u8] = include_bytes!("../../../env/cert.der");

#[derive(Deserialize)]
struct Input {
    email: Box<str>,
    password: Box<str>,
}

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("Make sure to send a post request with `application/json` content-type")]
    NoBody,

    #[error("Payload: {0}")]
    Payload(String),

    #[error("Rsa: {0}")]
    Rsa(String),

    #[error("Invalid Header Value: {0}")]
    InvalidHeaderValue(String),

    #[error("Reqwest: {0}")]
    Reqwest(String),

    #[error("Lambda HTTP: {0}")]
    LambdaHttp(String),
}

const METADATA: Metadata = Metadata {
    state: State::Texas,
    environment: Environment::Stage,
};

const URL: &str = METADATA.user_service_url();

const DEFAULT_HEADER: HeaderValue = HeaderValue::from_static("multipart/related; type=\"application/xop+xml\"");

pub async fn proxy(event: Request) -> Result<Response<Body>, ProxyError> {
    let input = event
        .payload::<Input>()
        .map_err(|err| ProxyError::Payload(err.to_string()))?
        .ok_or(ProxyError::NoBody)?;
    let req = AuthenticateUserRequest {
        email: input.email.as_ref(),
        password: input.password.as_ref(),
    };

    let rsa_private_key =
        RsaPrivateKey::from_pkcs8_pem(PKEY_PEM).map_err(|err| ProxyError::Rsa(err.to_string()))?;
    let signing_key = SigningKey::new(rsa_private_key);

    let multipart = req.efr_request(&signing_key, CERT_DER);
    let ct = HeaderValue::from_str(multipart.content_type())
        .map_err(|err| ProxyError::InvalidHeaderValue(err.to_string()))?;

    let client = reqwest::Client::builder()
        .build()
        .map_err(|err| ProxyError::Reqwest(err.to_string()))?;
    let resp = client
        .post(URL)
        .header(CONTENT_TYPE, ct)
        .header(
            AuthenticateUserRequest::SOAP_ACTION_HEADER_NAME,
            AuthenticateUserRequest::SOAP_ACTION,
        )
        .body(multipart.into_bytes())
        .send()
        .await
        .map_err(|err| ProxyError::Reqwest(err.to_string()))?;

    let status = resp.status();
    let headers = resp.headers();
    let content_type = headers.get(CONTENT_TYPE).cloned().unwrap_or(DEFAULT_HEADER);
    let message = resp.bytes().await.map_err(|err| ProxyError::Reqwest(err.to_string()))?;

    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, content_type)
        .body(message.as_ref().into())
        .map_err(|err| ProxyError::LambdaHttp(err.to_string()))
}

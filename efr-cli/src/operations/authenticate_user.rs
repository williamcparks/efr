use efr::{
    api::{EfrRequest, EfrResponse},
    user_service::{AuthenticateUserRequest, AuthenticateUserResponse},
};
use reqwest::{
    Client,
    header::{CONTENT_TYPE, HeaderValue},
};

use crate::{
    config::EfrConfig,
    operations::{METADATA, error::OperationsError},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<AuthedUser, OperationsError> {
    let authenticate_user_request = AuthenticateUserRequest {
        email: config.email.as_str(),
        password: config.password.as_str(),
    };

    let multipart =
        authenticate_user_request.efr_request(&config.signing_key, config.cert_der.as_slice());
    let ct = HeaderValue::from_str(multipart.content_type())?;

    let bytes = multipart.into_bytes();
    println!(
        "---outbound---\n{}\n---/outbound---",
        String::from_utf8_lossy(bytes.as_slice())
    );

    let res = client
        .post(METADATA.user_service_url())
        .header(CONTENT_TYPE, ct)
        .header(
            AuthenticateUserRequest::SOAP_ACTION_HEADER_NAME,
            AuthenticateUserRequest::SOAP_ACTION,
        )
        .body(bytes)
        .send()
        .await?;

    println!("---inbound---\n{res:#?}\n");
    let text = res.text().await?;
    println!("{text}");

    let response = AuthenticateUserResponse::efr_response(text.as_str())?;
    println!("{response:#?}");
    println!("---/inbound---");

    Ok(AuthedUser {
        email: response.email.to_owned(),
        password_hash: response.password_hash.to_owned(),
    })
}

pub struct AuthedUser {
    pub email: String,
    pub password_hash: String,
}

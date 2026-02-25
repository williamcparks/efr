use efr::{
    api::{EfrRequest, EfrResponse},
    court_record_service::{GetCaseRequest, GetCaseResponse},
};
use reqwest::{
    Client,
    header::{CONTENT_TYPE, HeaderValue},
};

use crate::{
    config::EfrConfig,
    operations::{METADATA, authenticate_user, error::OperationsError},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let jurisdiction =
        inquire::Text::new("What Jurisdiction Code Are You Searching In?").prompt()?;
    let case_tracking_id = inquire::Text::new("What Case Tracking ID?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_case_request = GetCaseRequest {
        email: authed_user.email.as_str(),
        password_hash: authed_user.password_hash.as_str(),
        efsp_url: "https://efile.click",
        jurisdiction: jurisdiction.as_str(),
        case_tracking_id: case_tracking_id.as_str(),
    };

    let multipart = get_case_request.efr_request(&config.signing_key, config.cert_der.as_slice());
    let ct = HeaderValue::from_str(multipart.content_type())?;

    let bytes = multipart.into_bytes();
    println!(
        "---outbound---\n{}\n---/outbound---",
        String::from_utf8_lossy(bytes.as_slice())
    );

    let res = client
        .post(METADATA.court_record_service_url())
        .header(CONTENT_TYPE, ct)
        .header(
            GetCaseRequest::SOAP_ACTION_HEADER_NAME,
            GetCaseRequest::SOAP_ACTION,
        )
        .body(bytes)
        .send()
        .await?;

    println!("---inbound---\n{res:#?}\n");
    let text = res.text().await?;
    println!("{text}");

    let response = GetCaseResponse::efr_response(text.as_str())?;
    println!("{response:#?}");
    println!("---/inbound---");

    Ok(())
}

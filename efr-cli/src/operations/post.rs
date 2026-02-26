use efr::api::EfrRequest;
use reqwest::{
    Client,
    header::{CONTENT_TYPE, HeaderValue},
};

use crate::{config::EfrConfig, operations::error::OperationsError};

pub async fn post<T: EfrRequest>(
    client: Client,
    config: &EfrConfig,
    request: &T,
    url: &str,
) -> Result<String, OperationsError> {
    let multipart = request.efr_request(&config.signing_key, config.cert_der.as_slice());
    let ct = HeaderValue::from_str(multipart.content_type())?;

    println!("---outbound---");
    println!("POST {url}");
    println!(
        "{CONTENT_TYPE}: {}",
        ct.to_str().unwrap_or("Invalid Content Type")
    );
    println!("{}: {}", T::SOAP_ACTION_HEADER_NAME, T::SOAP_ACTION,);

    let bytes = multipart.into_bytes();
    println!(
        "\n{}\n---/outbound---",
        String::from_utf8_lossy(bytes.as_slice())
    );

    let res = client
        .post(url)
        .header(CONTENT_TYPE, ct)
        .header(T::SOAP_ACTION_HEADER_NAME, T::SOAP_ACTION)
        .body(bytes)
        .send()
        .await?;

    println!("---inbound---\n{res:#?}\n");
    let text = res.text().await?;
    println!("{text}");
    println!("---/inbound---");

    Ok(text)
}

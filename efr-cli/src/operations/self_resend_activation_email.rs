use efr::{api::json, user_service::SelfResendActivationEmailRequest};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let self_resend_activation_email_request = SelfResendActivationEmailRequest {
        email: config.email.as_ref(),
    };

    let xml = post(
        client,
        config,
        &self_resend_activation_email_request,
        config.metadata.user_service_url(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    println!("{json_res:#?}");

    Ok(())
}

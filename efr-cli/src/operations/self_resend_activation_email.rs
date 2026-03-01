use efr::user_service::SelfResendActivationEmailRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let self_resend_activation_email_request = SelfResendActivationEmailRequest {
        email: config.email.as_ref(),
    };

    post(
        client,
        config,
        &self_resend_activation_email_request,
        config.metadata.user_service_url(),
    )
    .await?;

    Ok(())
}

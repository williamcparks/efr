use efr::user_service::ResetPasswordRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{METADATA, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let reset_password_request = ResetPasswordRequest {
        email: config.email.as_ref(),
    };

    post(
        client,
        config,
        &reset_password_request,
        METADATA.user_service_url(),
    )
    .await?;

    Ok(())
}

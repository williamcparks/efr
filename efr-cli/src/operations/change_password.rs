use efr::user_service::ChangePasswordRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{METADATA, authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let new_password = inquire::Text::new("New Password?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let change_password_request = ChangePasswordRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
        old_password: config.password.as_ref(),
        new_password: new_password.as_str(),
    };

    post(
        client,
        config,
        &change_password_request,
        METADATA.user_service_url(),
    )
    .await?;

    Ok(())
}

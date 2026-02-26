use efr::user_service::GetNotificationPreferencesRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{METADATA, authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_notification_preferences_request = GetNotificationPreferencesRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
    };

    post(
        client,
        config,
        &get_notification_preferences_request,
        METADATA.user_service_url(),
    )
    .await?;

    Ok(())
}

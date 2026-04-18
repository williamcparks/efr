use efr::{api::json, user_service::GetNotificationPreferencesRequest};
use reqwest::Client;
use serde_json::Value;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_notification_preferences_request = GetNotificationPreferencesRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
    };

    let xml = post(
        client,
        config,
        &get_notification_preferences_request,
        config.metadata.user_service_url(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    let json_res: Value = serde_json::from_slice(json_res.as_slice())?;
    println!("{json_res:#?}");

    Ok(())
}

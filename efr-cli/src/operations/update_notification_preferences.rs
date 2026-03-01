use efr::user_service::{NotificationPreferencesFlags, UpdateNotificationPreferencesRequest};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let mut flags = NotificationPreferencesFlags::empty();
    for (flag_name, corresponding_flag) in ARR {
        let prompt = format!("Enable notification preference: {flag_name}");
        let enable_flag = inquire::Confirm::new(prompt.as_str())
            .with_default(true)
            .prompt()?;
        if enable_flag {
            flags |= corresponding_flag;
        }
    }

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let update_notification_preferences_request = UpdateNotificationPreferencesRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
        flags,
    };

    post(
        client,
        config,
        &update_notification_preferences_request,
        config.metadata.user_service_url(),
    )
    .await?;

    Ok(())
}

const ARR: [(&str, NotificationPreferencesFlags); 8] = [
    ("ACCEPTED", NotificationPreferencesFlags::ACCEPTED),
    ("REJECTED", NotificationPreferencesFlags::REJECTED),
    ("SUBMITTED", NotificationPreferencesFlags::SUBMITTED),
    (
        "SERVICEUNDELIVERABLE",
        NotificationPreferencesFlags::SERVICEUNDELIVERABLE,
    ),
    (
        "SUBMISSIONFAILED",
        NotificationPreferencesFlags::SUBMISSIONFAILED,
    ),
    ("RECEIPTED", NotificationPreferencesFlags::RECEIPTED),
    (
        "RETURNFORCORRECTION",
        NotificationPreferencesFlags::RETURNFORCORRECTION,
    ),
    ("ACCOUNTLOCKED", NotificationPreferencesFlags::ACCOUNTLOCKED),
];

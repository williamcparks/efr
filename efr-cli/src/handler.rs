use strum::VariantArray;

use crate::{
    config::EfrConfig,
    error::CliError,
    operations::{
        Operations, authenticate_user, change_password, get_case, get_case_list,
        get_notification_preferences, get_policy, get_user, reset_password,
        self_resend_activation_email, update_notification_preferences, update_user,
    },
};

pub async fn handler() -> Result<(), CliError> {
    let config = EfrConfig::try_new()?;
    let client = reqwest::ClientBuilder::new().build()?;

    let operation = inquire::Select::new(
        "What Operation Are You Looking To Preform?",
        Operations::VARIANTS.to_vec(),
    )
    .prompt()?;

    match operation {
        Operations::GetCaseList => get_case_list::handler(client, &config).await?,
        Operations::GetCase => get_case::handler(client, &config).await?,
        Operations::GetPolicy => get_policy::handler(client, &config).await?,
        Operations::AuthenticateUser => {
            authenticate_user::handler(client, &config).await?;
        }
        Operations::ChangePassword => change_password::handler(client, &config).await?,
        Operations::GetUser => get_user::handler(client, &config).await?,
        Operations::UpdateUser => update_user::handler(client, &config).await?,
        Operations::ResetPassword => reset_password::handler(client, &config).await?,
        Operations::GetNotificationPreferences => {
            get_notification_preferences::handler(client, &config).await?
        }
        Operations::UpdateNotificationPreferences => {
            update_notification_preferences::handler(client, &config).await?
        }
        Operations::SelfResendActivationEmail => {
            self_resend_activation_email::handler(client, &config).await?
        }
    }

    Ok(())
}

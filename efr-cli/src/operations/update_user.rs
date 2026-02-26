use efr::user_service::UpdateUserRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{METADATA, authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let email_prompt = format!("Email ({})?", authed_user.email);
    let new_email = inquire::Text::new(email_prompt.as_str())
        .with_default(authed_user.email.as_ref())
        .prompt()?;

    let first_name_prompt = format!("First Name ({})?", authed_user.first_name);
    let first_name = inquire::Text::new(first_name_prompt.as_str())
        .with_default(authed_user.first_name.as_ref())
        .prompt()?;

    let middle_name = inquire::Text::new("Middle Name?").prompt()?;

    let last_name_prompt = format!("Last Name ({})?", authed_user.last_name);
    let last_name = inquire::Text::new(last_name_prompt.as_str())
        .with_default(authed_user.last_name.as_ref())
        .prompt()?;

    let update_user_request = UpdateUserRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),

        user_id: authed_user.user_id.as_ref(),
        new_email: new_email.as_str(),
        first_name: first_name.as_str(),
        middle_name: middle_name.as_str(),
        last_name: last_name.as_str(),
    };

    post(
        client,
        config,
        &update_user_request,
        METADATA.user_service_url(),
    )
    .await?;

    Ok(())
}

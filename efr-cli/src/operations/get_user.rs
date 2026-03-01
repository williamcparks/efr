use efr::user_service::GetUserRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_user_request = GetUserRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
        user_id: authed_user.user_id.as_ref(),
    };

    post(
        client,
        config,
        &get_user_request,
        config.metadata.user_service_url(),
    )
    .await?;

    Ok(())
}

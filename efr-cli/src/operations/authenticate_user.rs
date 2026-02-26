use efr::{
    api::EfrResponse,
    user_service::{AuthenticateUserRequest, AuthenticateUserResponse},
};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{METADATA, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<AuthedUser, OperationsError> {
    let authenticate_user_request = AuthenticateUserRequest {
        email: config.email.as_ref(),
        password: config.password.as_ref(),
    };

    let text = post(
        client,
        config,
        &authenticate_user_request,
        METADATA.user_service_url(),
    )
    .await?;

    let response = AuthenticateUserResponse::efr_response(text.as_str())?;
    println!("{response:#?}");

    Ok(AuthedUser {
        user_id: response.user_id.into(),
        email: response.email.into(),
        first_name: response.first_name.into(),
        last_name: response.last_name.into(),
        password_hash: response.password_hash.into(),
    })
}

pub struct AuthedUser {
    pub user_id: Box<str>,
    pub email: Box<str>,
    pub first_name: Box<str>,
    pub last_name: Box<str>,
    pub password_hash: Box<str>,
}

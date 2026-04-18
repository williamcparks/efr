use efr::{api::json, user_service::ResetPasswordRequest};
use reqwest::Client;
use serde_json::Value;

use crate::{
    config::EfrConfig,
    operations::{error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let reset_password_request = ResetPasswordRequest {
        email: config.email.as_ref(),
    };

    let xml = post(
        client,
        config,
        &reset_password_request,
        config.metadata.user_service_url(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    let json_res: Value = serde_json::from_slice(json_res.as_slice())?;
    println!("{json_res:#?}");

    Ok(())
}

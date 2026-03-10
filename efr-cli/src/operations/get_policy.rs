use efr::{api::json, court_policy_service::GetPolicyRequest};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let jurisdiction =
        inquire::Text::new("What Jurisdiction Code Are You Searching In?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_policy_request = GetPolicyRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
        efsp_url: config.efsp_url.as_str(),
        jurisdiction: jurisdiction.as_str(),
    };

    let xml = post(
        client,
        config,
        &get_policy_request,
        config.metadata.court_policy_service_url(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    println!("{json_res:?}");

    Ok(())
}

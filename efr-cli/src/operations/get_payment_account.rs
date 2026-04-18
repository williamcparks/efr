use efr::{api::json, firm_service::GetPaymentAccountRequest};
use reqwest::Client;
use serde_json::Value;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let payment_account_id = inquire::Text::new("Payment Account ID?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_payment_account_request = GetPaymentAccountRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),

        payment_account_id: payment_account_id.as_str(),
    };

    let xml = post(
        client,
        config,
        &get_payment_account_request,
        config.metadata.firm_service_url(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    let json_res: Value = serde_json::from_slice(json_res.as_slice())?;
    println!("{json_res:#?}");

    Ok(())
}

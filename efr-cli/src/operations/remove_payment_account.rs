use efr::firm_service::RemovePaymentAccountRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{METADATA, authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let payment_account_id = inquire::Text::new("Payment Account ID?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let remove_payment_account_request = RemovePaymentAccountRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),

        payment_account_id: payment_account_id.as_str(),
    };

    post(
        client,
        config,
        &remove_payment_account_request,
        METADATA.firm_service_url(),
    )
    .await?;

    Ok(())
}

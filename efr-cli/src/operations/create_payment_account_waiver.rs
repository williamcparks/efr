use efr::firm_service::CreatePaymentAccountRequestWaiver;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{METADATA, authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let account_name = inquire::Text::new("Account Name?").prompt()?;
    let payment_account_type_code = inquire::Text::new("Payment Account Type Code?").prompt()?;
    let payment_account_type_code_id =
        inquire::Text::new("Payment Account Type Code ID?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let create_payment_account_request_waiver = CreatePaymentAccountRequestWaiver {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),

        account_name: account_name.as_str(),
        payment_account_type_code: payment_account_type_code.as_str(),
        payment_account_type_code_id: payment_account_type_code_id.as_str(),
    };

    post(
        client,
        config,
        &create_payment_account_request_waiver,
        METADATA.firm_service_url(),
    )
    .await?;

    Ok(())
}

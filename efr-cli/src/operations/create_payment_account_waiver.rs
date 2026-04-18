use efr::{api::json, firm_service::CreatePaymentAccountRequestWaiver};
use reqwest::Client;
use serde_json::Value;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
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

    let xml = post(
        client,
        config,
        &create_payment_account_request_waiver,
        config.metadata.firm_service_url(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    let json_res: Value = serde_json::from_slice(json_res.as_slice())?;
    println!("{json_res:#?}");

    Ok(())
}

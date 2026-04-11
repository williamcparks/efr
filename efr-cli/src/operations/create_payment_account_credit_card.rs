use efr::{api::json, firm_service::CreatePaymentAccountRequestCreditCard};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let account_name = inquire::Text::new("Account Name?").prompt()?;
    let payment_account_type_code = inquire::Text::new("Payment Account Type Code?").prompt()?;
    let payment_account_type_code_id =
        inquire::Text::new("Payment Account Type Code ID?").prompt()?;
    let account_token = inquire::Text::new("Account Token?").prompt()?;
    let card_type = inquire::Text::new("Card Type?").prompt()?;
    let card_last_4 = inquire::Text::new("Card Last 4?").prompt()?;
    let card_month = inquire::Text::new("Card Month?").prompt()?;
    let card_year = inquire::Text::new("Card Year?").prompt()?;
    let card_holder_name = inquire::Text::new("Card Holder Name?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let create_payment_account_request_waiver = CreatePaymentAccountRequestCreditCard {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),

        account_name: account_name.as_str(),
        payment_account_type_code: payment_account_type_code.as_str(),
        payment_account_type_code_id: payment_account_type_code_id.as_str(),
        account_token: account_token.as_str(),
        card_type: card_type.as_str(),
        card_last_4: card_last_4.as_str(),
        card_month: card_month.as_str(),
        card_year: card_year.as_str(),
        card_holder_name: card_holder_name.as_str(),
    };

    let xml = post(
        client,
        config,
        &create_payment_account_request_waiver,
        config.metadata.firm_service_url(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    println!("{json_res:#?}");

    Ok(())
}

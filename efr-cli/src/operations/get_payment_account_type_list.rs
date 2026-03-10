use efr::{api::json, firm_service::GetPaymentAccountTypeListRequest};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_payment_account_type_list_request = GetPaymentAccountTypeListRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
    };

    let xml = post(
        client,
        config,
        &get_payment_account_type_list_request,
        config.metadata.firm_service_url(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    println!("{json_res:#?}");

    Ok(())
}

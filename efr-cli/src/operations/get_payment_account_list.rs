use efr::firm_service::GetPaymentAccountListRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{METADATA, authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_payment_account_list_request = GetPaymentAccountListRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
    };

    post(
        client,
        config,
        &get_payment_account_list_request,
        METADATA.firm_service_url(),
    )
    .await?;

    Ok(())
}

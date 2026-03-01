use efr::{
    api::EfrResponse,
    court_record_service::{GetCaseListRequestCaseNumber, GetCaseListResponse},
};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let efsp_url = inquire::Text::new("EFile Service Provider URL")
        .with_default("https://efsp.efile")
        .prompt()?;
    let jurisdiction =
        inquire::Text::new("What Jurisdiction Code Are You Searching In?").prompt()?;
    let case_number = inquire::Text::new("What Case Number?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_case_list_request = GetCaseListRequestCaseNumber {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),
        efsp_url: efsp_url.as_str(),
        jurisdiction: jurisdiction.as_str(),
        case_number: case_number.as_str(),
    };

    let text = post(
        client,
        config,
        &get_case_list_request,
        config.metadata.court_record_service_url(),
    )
    .await?;

    let response = GetCaseListResponse::efr_response(text.as_str())?;
    println!("{response:#?}");

    Ok(())
}

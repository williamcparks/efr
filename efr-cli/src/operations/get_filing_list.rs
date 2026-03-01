use efr::filing_review_service::GetFilingListRequest;
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{
        authenticate_user,
        error::OperationsError,
        inquire_helpers::{DateRange, InquireEmptyIsNone},
        post::post,
    },
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let efsp_url = inquire::Text::new("EFile Service Provider URL")
        .with_default("https://efsp.efile")
        .prompt()?;
    let jurisdiction =
        inquire::Text::new("Filter By Jurisdiction (optional)?").prompt_empty_is_none()?;
    let filing_status =
        inquire::Text::new("Filter By Filing Status (optional)?").prompt_empty_is_none()?;
    let case_tracking_id =
        inquire::Text::new("Filter By Case Tracking ID (optional)?").prompt_empty_is_none()?;
    let case_number =
        inquire::Text::new("Filter By Case Number (optional)?").prompt_empty_is_none()?;
    let date_range = match inquire::Confirm::new("Filter By Date Range?").prompt()? {
        true => Some(DateRange::prompt("Start Date?", "End Date")?),
        false => None,
    };
    let envelope_number =
        inquire::Text::new("Filter By Envelope Number (optional)?").prompt_empty_is_none()?;
    let submitter =
        inquire::Text::new("Filter By Submitter ID (optional)?").prompt_empty_is_none()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_case_list_request = GetFilingListRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),

        efsp_url: efsp_url.as_str(),

        jurisdiction: jurisdiction.as_deref(),
        filing_status: filing_status.as_deref(),
        case_tracking_id: case_tracking_id.as_deref(),
        case_number: case_number.as_deref(),
        date_range: date_range.map(|v| (v.start_date, v.end_date)),
        envelope_number: envelope_number.as_deref(),
        submitter: submitter.as_deref(),
    };

    post(
        client,
        config,
        &get_case_list_request,
        config.metadata.filing_review_service(),
    )
    .await?;

    Ok(())
}

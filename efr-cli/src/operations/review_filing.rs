use base64::Engine;
use efr::{api::json, filing_review_service::ReviewFilingRequest};
use reqwest::Client;
use serde_json::Value;

use crate::{
    config::EfrConfig,
    operations::{authenticate_user, error::OperationsError, post::post},
};

pub async fn handler(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let file_path = inquire::Text::new("PDF File Path?").prompt()?;
    let file_path = config.cwd.join(file_path.as_str());
    let file_bytes = match std::fs::read(file_path.as_path()) {
        Ok(ok) => ok,
        Err(err) => {
            return Err(OperationsError::Read {
                path: file_path,
                source: err,
            });
        }
    };
    let base_64 = base64::engine::general_purpose::STANDARD.encode(file_bytes.as_slice());

    let jurisdiction = inquire::Text::new("Jurisdiction?").prompt()?;

    let case_category_code = inquire::Text::new("Case Category Code?").prompt()?;
    let case_type_code = inquire::Text::new("Case Type Code?").prompt()?;
    let filing_code = inquire::Text::new("Filing Code?").prompt()?;
    let filing_component_code = inquire::Text::new("Filing Component Code?").prompt()?;
    let document_type_code = inquire::Text::new("Document Type Code?").prompt()?;
    let motion_type_code = inquire::Text::new("Motion Type Code?").prompt()?;
    let filer_type_code = inquire::Text::new("Filer Type Code?").prompt()?;

    let document_description_text = inquire::Text::new("Document Description Text?").prompt()?;
    let page_count = inquire::Text::new("Page Count?").prompt()?;
    let original_file_name = inquire::Text::new("Original File Name?").prompt()?;

    let binary_description_text = inquire::Text::new("Binary Description Text?").prompt()?;
    let binary_size_value = file_bytes.len().to_string();
    let filing_comments = inquire::Text::new("Filing Comments?").prompt()?;

    let case_tracking_id = inquire::Text::new("Case Tracking ID?").prompt()?;

    let payment_account_id = inquire::Text::new("Payment Account ID?").prompt()?;

    let authed_user = authenticate_user::handler(client.clone(), config).await?;

    let get_case_list_request = ReviewFilingRequest {
        email: authed_user.email.as_ref(),
        password_hash: authed_user.password_hash.as_ref(),

        efsp_url: config.efsp_url.as_str(),
        jurisdiction: jurisdiction.as_str(),

        case_category_code: case_category_code.as_str(),
        case_type_code: case_type_code.as_str(),
        filing_code: filing_code.as_str(),
        filing_component_code: filing_component_code.as_str(),
        document_type_code: document_type_code.as_str(),
        motion_type_code: motion_type_code.as_str(),
        filer_type_code: filer_type_code.as_str(),

        document_description_text: document_description_text.as_str(),
        page_count: page_count.as_str(),
        original_file_name: original_file_name.as_str(),
        base_64: base_64.as_str(),
        binary_description_text: binary_description_text.as_str(),
        binary_size_value: binary_size_value.as_str(),
        filing_comments: filing_comments.as_str(),

        case_tracking_id: case_tracking_id.as_str(),

        payment_account_id: payment_account_id.as_str(),
    };

    let xml = post(
        client,
        config,
        &get_case_list_request,
        config.metadata.filing_review_service(),
    )
    .await?;

    let json_res = json(xml.as_str())?;
    let json_res: Value = serde_json::from_slice(json_res.as_slice())?;
    println!("{json_res:#?}");

    Ok(())
}

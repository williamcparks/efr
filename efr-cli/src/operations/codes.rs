use efr::codes_service::{CodeHeader, CodeResponse};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{
        error::OperationsError,
        get::{get, read},
        inquire_helpers::{CodeListPrompt, CodesSource},
    },
};

pub async fn codes(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let code_list = CodeListPrompt::prompt()?;
    let codes_source = CodesSource::prompt()?;
    let file_name = code_list.to_string();

    let bytes = match codes_source {
        CodesSource::Fetch => {
            let jurisdiction = match code_list.requires_jurisdiction() {
                true => Some(inquire::Text::new("Jurisdiction?").prompt()?),
                false => None,
            };
            let url = code_list.url(&config.metadata, jurisdiction.as_deref());
            get(client, config, url.as_ref(), file_name.as_str()).await?
        }
        CodesSource::LocalFs => read(file_name.as_str(), config.cwd.as_path())?,
    };

    let xml = CodeHeader::unzip_xml(bytes.as_ref())?;

    let code_response = CodeResponse::try_new(xml.as_str())?;

    for row_result in code_response {
        let row = row_result?;
        println!("{row:#?}");
    }

    Ok(())
}

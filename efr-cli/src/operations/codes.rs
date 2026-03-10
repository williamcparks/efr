use efr::codes_service::{CodeVersion, CodesHeader, CodesResponse};
use reqwest::Client;
use strum::Display;

use crate::{
    config::EfrConfig,
    operations::{
        error::OperationsError,
        get::{get, read},
    },
};

#[derive(Display)]
enum CodesSource {
    #[strum(to_string = "Fetch")]
    Fetch,
    #[strum(to_string = "Local File System")]
    LocalFs,
}

impl CodesSource {
    fn prompt() -> Result<Self, OperationsError> {
        let ok = inquire::Select::new(
            "How Would You Like To Pull Codes?",
            vec![Self::Fetch, Self::LocalFs],
        )
        .prompt()?;
        Ok(ok)
    }
}

pub async fn location(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let codes_source = CodesSource::prompt()?;
    let _bytes = match codes_source {
        CodesSource::Fetch => get(client, config, config.metadata.location_codes_url()).await?,
        CodesSource::LocalFs => read("locations.zip")?,
    };

    todo!();
}

pub async fn version(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let codes_source = CodesSource::prompt()?;
    let bytes = match codes_source {
        CodesSource::Fetch => get(client, config, config.metadata.version_codes_url()).await?,
        CodesSource::LocalFs => read("codeversions.zip")?,
    };

    let xml = CodesHeader::unzip_xml(bytes.as_ref())?;
    let codes_response = CodesResponse::<'_, CodeVersion>::try_new(xml.as_str())?;

    println!("{:#?}", codes_response.codes_metadata);
    for row_result in codes_response.rows {
        println!("{:#?}", row_result?);
    }

    Ok(())
}

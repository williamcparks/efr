use efr::codes_service::{CodeError, CodeHeader, CodeLocation, CodeResponse, CodeVersion};
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
    let bytes = match codes_source {
        CodesSource::Fetch => get(client, config, config.metadata.location_codes_url()).await?,
        CodesSource::LocalFs => read("locations.zip")?,
    };

    let xml = CodeHeader::unzip_xml(bytes.as_ref())?;
    let code_response = CodeResponse::<'_, CodeLocation>::try_new(xml.as_str())?;

    println!("{:#?}", code_response.codes_metadata);
    for row_result in code_response.rows {
        println!("{:#?}", row_result?);
    }

    Ok(())
}

pub async fn version(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let codes_source = CodesSource::prompt()?;
    let bytes = match codes_source {
        CodesSource::Fetch => get(client, config, config.metadata.version_codes_url()).await?,
        CodesSource::LocalFs => read("codeversions.zip")?,
    };

    let xml = CodeHeader::unzip_xml(bytes.as_ref())?;
    let code_response = CodeResponse::<'_, CodeVersion>::try_new(xml.as_str())?;

    println!("{:#?}", code_response.codes_metadata);
    for row_result in code_response.rows {
        println!("{:#?}", row_result?);
    }

    Ok(())
}

pub async fn error(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let codes_source = CodesSource::prompt()?;
    let bytes = match codes_source {
        CodesSource::Fetch => get(client, config, config.metadata.error_codes_url()).await?,
        CodesSource::LocalFs => read("errorcodes.zip")?,
    };

    let xml = CodeHeader::unzip_xml(bytes.as_ref())?;

    let code_response = CodeResponse::<'_, CodeError>::try_new(xml.as_str())?;

    println!("{:#?}", code_response.codes_metadata);
    for row_result in code_response.rows {
        println!("{:#?}", row_result?);
    }

    Ok(())
}

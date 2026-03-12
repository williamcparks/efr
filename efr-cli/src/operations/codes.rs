use efr::codes_service::{
    CodeCountry, CodeDataField, CodeError, CodeFilingStatus, CodeHeader, CodeLocation,
    CodeResponse, CodeState, CodeVersion,
};
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

macro_rules! code_fetch {
    ($ident: ident, $file_name: literal, $url: ident, $ty: ty) => {
        pub async fn $ident(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
            let codes_source = CodesSource::prompt()?;
            let bytes = match codes_source {
                CodesSource::Fetch => get(client, config, config.metadata.$url()).await?,
                CodesSource::LocalFs => read($file_name)?,
            };

            let xml = CodeHeader::unzip_xml(bytes.as_ref())?;

            for line in xml.lines().take(100) {
                println!("{line}");
            }

            let code_response = CodeResponse::<'_, $ty>::try_new(xml.as_str())?;

            println!("{:#?}", code_response.codes_metadata);
            for row_result in code_response.rows {
                println!("{:#?}", row_result?);
            }

            Ok(())
        }
    };
}

code_fetch! {
    location, "locations.zip", location_codes_url, CodeLocation
}

code_fetch! {
    version, "codeversions.zip", version_codes_url, CodeVersion
}

code_fetch! {
    error, "errorcodes.zip", error_codes_url, CodeError
}

code_fetch! {
    country, "countrycodes.zip", country_codes_url, CodeCountry
}

code_fetch! {
    state, "statecodes.zip", state_codes_url, CodeState
}

code_fetch! {
    filing_status, "filingstatuscodes.zip", filing_status_codes_url, CodeFilingStatus
}

code_fetch! {
    data_field, "datafieldcodes.zip", data_field_codes_url, CodeDataField
}

use efr::codes_service::{
    CodeCaseCategory, CodeCaseType, CodeCountry, CodeDataField, CodeDocumentType, CodeError,
    CodeFilerType, CodeFiling, CodeFilingComponent, CodeFilingStatus, CodeHeader, CodeLocation,
    CodeMotionType, CodeResponse, CodeState, CodeVersion,
};
use reqwest::Client;
use strum::Display;

use crate::{
    config::EfrConfig,
    operations::{
        error::OperationsError,
        get::{get, read},
        inquire_helpers::InquireEmptyIsNone,
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
                CodesSource::LocalFs => read($file_name, config.cwd.as_path())?,
            };

            let xml = CodeHeader::unzip_xml(bytes.as_ref())?;

            let code_response = CodeResponse::<'_, $ty>::try_new(xml.as_str())?;

            println!("{:#?}", code_response.codes_metadata);
            for row_result in code_response.rows {
                println!("{:#?}", row_result?);
            }

            Ok(())
        }
    };

    ($ident: ident, $file_name: literal, $url: ident (_), $ty: ty) => {
        pub async fn $ident(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
            let jurisdiction = inquire::Text::new("Jurisdiction?").prompt()?;
            let url = config.metadata.$url(jurisdiction.as_str());

            let codes_source = CodesSource::prompt()?;
            let bytes = match codes_source {
                CodesSource::Fetch => get(client, config, url.as_str()).await?,
                CodesSource::LocalFs => read($file_name, config.cwd.as_path())?,
            };

            let xml = CodeHeader::unzip_xml(bytes.as_ref())?;

            let code_response = CodeResponse::<'_, $ty>::try_new(xml.as_str())?;

            println!("{:#?}", code_response.codes_metadata);
            for row_result in code_response.rows {
                println!("{:#?}", row_result?);
            }

            Ok(())
        }
    };

    ($ident: ident, $file_name: literal, $url: ident (_), $ty: ty, $filter_prompt: literal, $filter_field: ident) => {
        pub async fn $ident(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
            let jurisdiction = inquire::Text::new("Jurisdiction?").prompt()?;
            let filter = inquire::Text::new($filter_prompt).prompt_empty_is_none()?;
            let url = config.metadata.$url(jurisdiction.as_str());

            let codes_source = CodesSource::prompt()?;
            let bytes = match codes_source {
                CodesSource::Fetch => get(client, config, url.as_str()).await?,
                CodesSource::LocalFs => read($file_name, config.cwd.as_path())?,
            };

            let xml = CodeHeader::unzip_xml(bytes.as_ref())?;

            let code_response = CodeResponse::<'_, $ty>::try_new(xml.as_str())?;

            println!("{:#?}", code_response.codes_metadata);
            for row_result in code_response.rows {
                let row = row_result?;
                if let Some(fil_condition) = filter.as_deref()
                    && row.$filter_field.as_ref() != fil_condition
                {
                    continue;
                }
                println!("{row:#?}");
            }

            Ok(())
        }
    };
}

code_fetch! {
    location, "locations.zip", locations_url, CodeLocation
}

code_fetch! {
    version, "codeversions.zip", version_url, CodeVersion
}

code_fetch! {
    error, "errorcodes.zip", error_url, CodeError
}

code_fetch! {
    country, "countrycodes.zip", country_url, CodeCountry
}

code_fetch! {
    state, "statecodes.zip", state_url, CodeState
}

code_fetch! {
    filing_status, "filingstatuscodes.zip", filing_status_url, CodeFilingStatus
}

code_fetch! {
    data_field, "datafieldcodes.zip", data_field_config_url, CodeDataField
}

code_fetch! {
    case_category, "casecategorycodes.zip", case_category_url(_), CodeCaseCategory
}

code_fetch! {
    case_type, "casetypecodes.zip", case_type_url(_), CodeCaseType
}

code_fetch! {
    filing, "filingcodes.zip", filing_url(_), CodeFiling, "Case Category Filter (Optional)?", casecategory
}

code_fetch! {
    motion_type, "motiontypecodes.zip", motion_type_url(_), CodeMotionType
}

code_fetch! {
    filing_component, "filingcomponentcodes.zip", filing_component_url(_), CodeFilingComponent, "Filing Code Filter (Optional)?", filingcodeid
}

code_fetch! {
    document_type, "documenttypecodes.zip", document_type_url(_), CodeDocumentType
}

code_fetch! {
    filer_type, "filertypecodes.zip", filer_type_url(_), CodeFilerType
}

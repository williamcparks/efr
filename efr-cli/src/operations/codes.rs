use efr::codes_service::{CodeHeader, CodeList, CodeResponse};
use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{
        error::OperationsError,
        get::{get, read},
        inquire_helpers::{CodeListPrompt, CodesSource, InquireEmptyIsNone},
    },
};

pub async fn codes(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    let code_list = CodeListPrompt::prompt()?;
    let filter = prompt_filter(code_list)?;
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

        let is_match = filter.iter().all(|(value_fil, columns_fil)| {
            columns_fil
                .iter()
                .find_map(|col| row.get(col))
                .and_then(|v| v.as_str())
                .map(|v| v == value_fil.as_ref())
                .unwrap_or_default()
        });

        if is_match {
            println!("{row:#?}");
        }
    }

    Ok(())
}

#[allow(clippy::type_complexity)]
pub fn prompt_filter(
    code_list: CodeList,
) -> Result<Vec<(Box<str>, &'static [&'static str])>, OperationsError> {
    let mut map = Vec::new();

    let queries: &'static [CodeList] = match code_list {
        CodeList::CaseType | CodeList::ProcedureRemedy | CodeList::DamageAmount => {
            &[CodeList::CaseCategory]
        }
        CodeList::CaseSubType | CodeList::CrossReference | CodeList::PartyType => {
            &[CodeList::CaseType]
        }
        CodeList::Filing => &[CodeList::CaseCategory, CodeList::CaseType],
        CodeList::DocumentType
        | CodeList::FilingComponent
        | CodeList::MotionType
        | CodeList::OptionalServices => &[CodeList::Filing],
        CodeList::Answer => &[CodeList::Question],
        CodeList::Degree | CodeList::GeneralOffense => &[CodeList::Statute],
        _ => &[],
    };

    for query in queries {
        let prompt = format!("Filter By {query:?} (Optional)?");

        if let Some(fil) = inquire::Text::new(prompt.as_str()).prompt_empty_is_none()? {
            let fields: &'static [&'static str] = match query {
                CodeList::CaseCategory => &["casecategory"],
                CodeList::CaseType => &["casetype", "casetypeid"],
                CodeList::Filing => &["filingcodeid"],
                CodeList::Question => &["questionid"],
                CodeList::Statute => &["statutecodeid"],
                _ => &[],
            };
            map.push((fil.into_boxed_str(), fields));
        }
    }

    Ok(map)
}

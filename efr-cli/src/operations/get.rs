use std::{path::PathBuf, str::FromStr};

use efr::codes_service::CodesHeader;
use mime::Mime;
use reqwest::{
    Client, Response,
    header::{CONTENT_DISPOSITION, HeaderValue},
};

use crate::{config::EfrConfig, operations::error::OperationsError};

pub async fn get(client: Client, config: &EfrConfig, url: &str) -> Result<(), OperationsError> {
    let codes_header = CodesHeader::try_new(config.cert_der.as_slice(), &config.codes_signing_key)?;
    let codes_header_value = HeaderValue::from_str(codes_header.as_str())?;

    println!("---outbound---");
    println!("GET {url}");
    println!(
        "{}: {}",
        CodesHeader::TYL_EFM_API_HEADER,
        codes_header.as_str()
    );
    println!("---/outbound---");

    let res = client
        .get(url)
        .header(CodesHeader::TYL_EFM_API_HEADER, codes_header_value)
        .send()
        .await?;

    println!("---inbound---\n{res:#?}\n");

    let output = output_file(&res)?;
    if let Err(err) = std::fs::write(output.as_path(), res.bytes().await?) {
        return Err(OperationsError::Write {
            path: output,
            source: err,
        });
    }

    println!("output zip written to {}", output.display());
    println!("---/inbound---");

    Ok(())
}

fn output_file(response: &Response) -> Result<PathBuf, OperationsError> {
    let mut out = std::env::current_dir().map_err(OperationsError::Cwd)?;

    if let Some(header) = response.headers().get(CONTENT_DISPOSITION)
        && let Ok(content_dispo) = header.to_str()
    {
        let mime_parsed = match Mime::from_str(content_dispo) {
            Ok(ok) => Ok(ok),
            _ => {
                let extended = format!("text/{content_dispo}");
                Mime::from_str(extended.as_str())
            }
        };

        if let Ok(mime) = mime_parsed
            && let Some(file_name) = mime.get_param("filename")
        {
            out.push(file_name.as_str());
            return Ok(out);
        }
    }

    out.push("out.zip");
    Ok(out)
}

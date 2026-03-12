use std::{io::ErrorKind, path::PathBuf, str::FromStr};

use bytes::Bytes;
use efr::codes_service::CodeHeader;
use mime::Mime;
use reqwest::{
    Client, Response,
    header::{CONTENT_DISPOSITION, HeaderValue},
};

use crate::{config::EfrConfig, operations::error::OperationsError};

pub async fn get(client: Client, config: &EfrConfig, url: &str) -> Result<Bytes, OperationsError> {
    let codes_header = CodeHeader::try_new(config.cert_der.as_slice(), &config.codes_signing_key)?;
    let codes_header_value = HeaderValue::from_str(codes_header.as_str())?;

    println!("---outbound---");
    println!("GET {url}");
    println!(
        "{}: {}",
        CodeHeader::TYL_EFM_API_HEADER,
        codes_header.as_str()
    );
    println!("---/outbound---");

    let res = client
        .get(url)
        .header(CodeHeader::TYL_EFM_API_HEADER, codes_header_value)
        .send()
        .await?;

    println!("---inbound---\n{res:#?}\n");

    let output = output_file(&res)?;
    let bytes = res.bytes().await?;
    if let Err(err) = std::fs::write(output.as_path(), bytes.as_ref()) {
        return Err(OperationsError::Write {
            path: output,
            source: err,
        });
    }

    println!("output zip written to {}", output.display());
    println!("---/inbound---");

    Ok(bytes)
}

fn output_file(response: &Response) -> Result<PathBuf, OperationsError> {
    let mut out = std::env::current_dir().map_err(OperationsError::Cwd)?;
    out.push("code-downloads");

    if let Err(err) = std::fs::create_dir(out.as_path())
        && err.kind() != ErrorKind::AlreadyExists
    {
        return Err(OperationsError::Write {
            path: out,
            source: err,
        });
    }

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

    out.push("codes.zip");
    Ok(out)
}

pub fn read(file: &str) -> Result<Bytes, OperationsError> {
    let mut cwd = std::env::current_dir().map_err(OperationsError::Cwd)?;
    cwd.push("code-downloads");
    cwd.push(file);

    match std::fs::read(cwd.as_path()) {
        Ok(ok) => Ok(Bytes::from(ok)),
        Err(err) => Err(OperationsError::Read {
            path: cwd,
            source: err,
        }),
    }
}

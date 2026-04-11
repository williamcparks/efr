use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use bytes::Bytes;
use efr::codes_service::CodeHeader;
use reqwest::{Client, header::HeaderValue};

use crate::{config::EfrConfig, operations::error::OperationsError};

pub async fn get(
    client: Client,
    config: &EfrConfig,
    url: &str,
    file_name: &str,
) -> Result<Bytes, OperationsError> {
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

    let output = output_file(config.cwd.as_path(), file_name)?;
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

fn output_file(cwd: &Path, file_name: &str) -> Result<PathBuf, OperationsError> {
    let mut out = cwd.to_path_buf();
    out.push("code-downloads");

    if let Err(err) = std::fs::create_dir(out.as_path())
        && err.kind() != ErrorKind::AlreadyExists
    {
        return Err(OperationsError::Write {
            path: out,
            source: err,
        });
    }

    out.push(file_name);
    Ok(out)
}

pub fn read(file_name: &str, cwd: &Path) -> Result<Bytes, OperationsError> {
    let mut path = cwd.to_path_buf();
    path.push("code-downloads");
    path.push(file_name);

    match std::fs::read(path.as_path()) {
        Ok(ok) => Ok(Bytes::from(ok)),
        Err(err) => Err(OperationsError::Read { path, source: err }),
    }
}

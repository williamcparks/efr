use std::path::PathBuf;

use efr::{
    api::EfrError,
    codes_service::{EfrCodesError, EfrCodesHeaderError},
};
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum OperationsError {
    #[error("Request Error: {0}")]
    Reqwest(
        #[from]
        #[source]
        reqwest::Error,
    ),

    #[error("Failed to create `Content-Type` header: {0}")]
    ContentType(
        #[from]
        #[source]
        InvalidHeaderValue,
    ),

    Efr(#[from] EfrError),

    CodesHeader(#[from] EfrCodesHeaderError),

    Codes(#[from] EfrCodesError),

    Inquire(#[from] inquire::InquireError),

    Json(#[from] serde_json::Error),

    #[error("Failed To Write To `{}` Due To: {source}", .path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed To Read `{}` Due To: {source}", .path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

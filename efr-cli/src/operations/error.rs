use efr::api::EfrError;
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
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

    #[error(transparent)]
    Efr(#[from] EfrError),

    #[error(transparent)]
    Inquire(#[from] inquire::InquireError),
}

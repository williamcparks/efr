use lambda_http::ext::PayloadError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SendError {
    #[error("404 {0}")]
    UnknownRoute(Box<str>),

    Json(#[from] serde_json::Error),

    Payload(#[from] PayloadError),

    ReqwestClient(reqwest::Error),

    #[error("Header: `state`: {0}")]
    StateHeader(lambda_http::http::header::ToStrError),

    #[error("Header: `state`: No State Header Provided")]
    NoStateHeader,

    #[error("Header: `environment`: {0}")]
    EnviroHeader(lambda_http::http::header::ToStrError),

    #[error("Header: `environment`: No State Header Provided")]
    NoEnviroHeader,
}

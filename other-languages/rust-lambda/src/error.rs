use lambda_http::{ext::PayloadError, http::Method};
use thiserror::Error;

use crate::send::SendError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ProxyError {
    #[error("404 - {0}")]
    UnknownRoute(Box<str>),

    Json(#[from] serde_json::Error),

    Qs(#[from] serde_qs::Error),

    Response(lambda_http::Error),

    Payload(PayloadError),

    #[error("Reqwest Client - {0}")]
    ReqwestClient(reqwest::Error),

    Reqwest(reqwest::Error),

    #[error("Header: `state`: {0}")]
    StateHeader(lambda_http::http::header::ToStrError),

    #[error("Header: `state`: No State Header Provided")]
    NoStateHeader,

    #[error("Header: `environment`: {0}")]
    EnviroHeader(lambda_http::http::header::ToStrError),

    #[error("Header: `environment`: No State Header Provided")]
    NoEnviroHeader,

    State(efr::api::StateError),

    Enviro(efr::api::EnvironmentError),

    #[error("Invalid Auth Token, Must Be {{email}}:{{password}}")]
    AuthToken,

    #[error("Use Method: `{0}`")]
    UseMethod(Method),

    #[error("RSA - {0}")]
    Rsa(Box<str>),

    Efr(efr::api::EfrError),
}

impl From<SendError> for ProxyError {
    fn from(value: SendError) -> Self {
        match value {
            SendError::UnknownRoute(err) => Self::UnknownRoute(err),
            SendError::Json(err) => Self::Json(err),
            SendError::Qs(err) => Self::Qs(err),
            SendError::Payload(err) => Self::Payload(err),
            SendError::ReqwestClient(err) => Self::ReqwestClient(err),
            SendError::Reqwest(err) => Self::Reqwest(err),
            SendError::StateHeader(err) => Self::StateHeader(err),
            SendError::NoStateHeader => Self::NoStateHeader,
            SendError::EnviroHeader(err) => Self::EnviroHeader(err),
            SendError::NoEnviroHeader => Self::NoEnviroHeader,
            SendError::State(err) => Self::State(err),
            SendError::Enviro(err) => Self::Enviro(err),
            SendError::AuthToken => Self::AuthToken,
            SendError::UseMethod(err) => Self::UseMethod(err),
            SendError::Rsa(err) => Self::Rsa(err),
            SendError::Efr(err) => Self::Efr(err),
        }
    }
}

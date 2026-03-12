use lambda_http::{ext::PayloadError, http::Method};
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SendError {
    #[error("404 {0}")]
    UnknownRoute(Box<str>),

    Json(#[from] serde_json::Error),

    Qs(#[from] serde_qs::Error),

    Payload(#[from] PayloadError),

    ReqwestClient(reqwest::Error),

    Reqwest(#[from] reqwest::Error),

    #[error("Header: `state`: {0}")]
    StateHeader(lambda_http::http::header::ToStrError),

    #[error("Header: `state`: No State Header Provided")]
    NoStateHeader,

    #[error("Header: `environment`: {0}")]
    EnviroHeader(lambda_http::http::header::ToStrError),

    #[error("Header: `environment`: No State Header Provided")]
    NoEnviroHeader,

    State(#[from] efr::api::StateError),

    Enviro(#[from] efr::api::EnvironmentError),

    #[error("Invalid Auth Token, Must Be {{email}}:{{password}}")]
    AuthToken,

    #[error("Use Method: `{0}`")]
    UseMethod(Method),

    #[error("RSA - {0}")]
    Rsa(Box<str>),

    Efr(#[from] efr::api::EfrError),
}

use thiserror::Error;

use crate::{config::ConfigError, operations::error::OperationsError};

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("Failed create http client due to: {0}")]
    ReqwestClient(
        #[from]
        #[source]
        reqwest::Error,
    ),

    #[error(transparent)]
    Inquire(#[from] inquire::InquireError),

    #[error(transparent)]
    Operations(#[from] OperationsError),
}

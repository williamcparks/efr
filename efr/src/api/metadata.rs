use efr_macros::metadata;
use thiserror::Error;

metadata! {}

#[derive(Clone, Debug, Error)]
#[error("Available Environments: stage, production")]
pub struct EnvironmentError;

impl Environment {
    pub fn try_new(environment: &str) -> Result<Self, EnvironmentError> {
        match environment {
            "stage" => Ok(Self::Stage),
            "production" => Ok(Self::Production),
            _ => Err(EnvironmentError),
        }
    }
}

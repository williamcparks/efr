use efr::api::{Environment, Metadata, State};
use lambda_http::{http::HeaderValue, Request, RequestPayloadExt};
use serde::de::DeserializeOwned;

use crate::send::{AuthenticateUser, SendError};

pub enum Input {
    Authenticate(AuthenticateUser),
}

impl Input {
    pub fn try_new(request: Request) -> Result<(Self, Metadata), SendError> {
        let metadata = try_metadata(&request)?;

        let url = request.uri().path();

        match url {
            "/authenticate" => Ok((Self::Authenticate(Self::require_body(request)?), metadata)),
            other => Err(SendError::UnknownRoute(other.into())),
        }
    }

    fn require_body<T: DeserializeOwned>(request: Request) -> Result<T, SendError> {
        match request.payload()? {
            Some(some) => Ok(some),
            None => Ok(serde_json::from_str("")?),
        }
    }
}

fn try_metadata(request: &Request) -> Result<Metadata, SendError> {
    let headers = request.headers();
    let state = match headers.get("state").map(HeaderValue::to_str) {
        Some(Ok(ok)) => ok,
        Some(Err(err)) => return Err(SendError::StateHeader(err)),
        None => return Err(SendError::NoStateHeader),
    };
    let enviro = match headers.get("environment").map(HeaderValue::to_str) {
        Some(Ok(ok)) => ok,
        Some(Err(err)) => return Err(SendError::EnviroHeader(err)),
        None => return Err(SendError::NoEnviroHeader),
    };

    let state = State::try_new(state)?;
    let environment = Environment::try_new(state)?;

    Ok(Metadata { state, environment })
}

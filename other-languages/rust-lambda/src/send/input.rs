use efr::api::{Environment, Metadata, State};
use lambda_http::{http::HeaderValue, Request, RequestPayloadExt};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::send::{AuthToken, AuthenticateUser, GetCaseList, SendError};

pub enum Input {
    Authenticate(AuthenticateUser),
    GetCaseList(GetCaseList),
}

impl Input {
    pub fn try_new(request: Request) -> Result<(Self, Metadata, Option<AuthToken>), SendError> {
        let metadata = try_metadata(&request)?;
        let authtoken = AuthToken::new_opt(&request);

        let url = request.uri().path();

        match url {
            "/authenticate" => Ok((
                Self::Authenticate(Self::require_body(request, Method::POST)?),
                metadata,
                authtoken,
            )),
            "/get_case_list" => Ok((
                Self::GetCaseList(Self::require_qs(request, Method::GET)?),
                metadata,
                authtoken,
            )),
            other => Err(SendError::UnknownRoute(other.into())),
        }
    }

    fn require_body<T: DeserializeOwned>(request: Request, method: Method) -> Result<T, SendError> {
        if request.method() != method {
            return Err(SendError::UseMethod(method));
        }

        match request.payload()? {
            Some(some) => Ok(some),
            None => Ok(serde_json::from_value(Value::Null)?),
        }
    }

    fn require_qs<T: DeserializeOwned>(request: Request, method: Method) -> Result<T, SendError> {
        if request.method() != method {
            return Err(SendError::UseMethod(method));
        }

        let query = request.uri().query().unwrap_or("");
        Ok(serde_qs::from_str(query)?)
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
    let environment = Environment::try_new(enviro)?;

    Ok(Metadata { state, environment })
}

use efr::api::Metadata;
use reqwest::Client;
use serde_json::Value;

use crate::send::{AuthToken, Input, SendError};

pub async fn handler(
    input: Input,
    metadata: Metadata,
    authtoken: Option<AuthToken>,
) -> Result<Value, SendError> {
    let client = Client::builder()
        .build()
        .map_err(SendError::ReqwestClient)?;

    match input {
        Input::Authenticate(req) => req.handler(client, metadata).await,
        Input::GetCaseList(req) => req.handler(client, metadata, authtoken).await,
    }
}

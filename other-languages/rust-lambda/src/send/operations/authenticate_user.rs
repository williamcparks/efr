use efr::api::Metadata;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::send::SendError;

#[derive(Deserialize)]
pub struct AuthenticateUser {
    pub email: Box<str>,
    pub password: Box<str>,
}

impl AuthenticateUser {
    pub async fn handler(&self, client: Client, metadata: Metadata) -> Result<Value, SendError> {
        todo!()
    }
}

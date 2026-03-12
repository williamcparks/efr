use lambda_http::{Body, Request, Response};

use crate::{
    error::ProxyError,
    send::{handler as send, Input},
};

pub async fn proxy(event: Request) -> Result<Response<Body>, ProxyError> {
    let (input, metadata) = Input::try_new(event)?;

    let json_obj = send(input, metadata).await?;
    let json_msg = serde_json::to_string(&json_obj)?;

    Response::builder()
        .status(200)
        .body(json_msg.into())
        .map_err(|err| ProxyError::Response(Box::new(err)))
}

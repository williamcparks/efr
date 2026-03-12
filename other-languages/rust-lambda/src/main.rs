use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde_json::json;

mod error;
mod proxy;
mod send;

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    match proxy::proxy(event).await {
        Ok(ok) => Ok(ok),
        Err(err) => {
            let err_msg = err.to_string();
            let json_obj = json!({ "err": err_msg });
            let json_msg = serde_json::to_string(&json_obj)?;

            let resp = Response::builder()
                .status(400)
                .body(json_msg.into())
                .map_err(Box::new)?;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(handler)).await
}

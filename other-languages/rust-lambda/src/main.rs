use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

mod proxy;

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    match proxy::proxy(event).await {
        Ok(ok) => Ok(ok),
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .body(err.to_string().into())
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

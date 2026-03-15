# Rust Lambda EFile Proxy

setup

- make sure you followed instructions in `cli.md` for setting up _/env_ folder in the root
- once _/env_ folder is setup this lambda will build, it assumes:
- env/pkey.pem exists
- env/cert.der exists

## Metadata in requests

You need to add the state, environment, and optionally an "authtoken" for authenticated routes to access the lambda

HTTP Headers:

- state = lower case abbreviation of the state you are using, for example "tx" for Texas or "ca" for California
- environment = lower case of either "stage" or "production"
- authtoken = email:password-hash where "password-hash" comes from the /authenticate api for example:

```
POST /authenticate
state: tx
environment: stage
content-type: application/json

{
  "email": "user@example.com"
  "password": "password"
}
```

```
GET /get_case_list?case_number=example&jurisdiction=juris:county
state: tx
environment: stage
authtoken: user@example.com:whatever-password-hash-from-auth-call
```

# Introduction

rust-lambda is a Rust project that implements an AWS Lambda function in Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)

## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

You can run regular Rust unit tests with `cargo test`.

If you want to run integration tests locally, you can use the `cargo lambda watch` and `cargo lambda invoke` commands to do it.

First, run `cargo lambda watch` to start a local server. When you make changes to the code, the server will automatically restart.

Second, you'll need a way to pass the event data to the lambda function.

You can use the existent [event payloads](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) in the Rust Runtime repository if your lambda function is using one of the supported event types.

You can use those examples directly with the `--data-example` flag, where the value is the name of the file in the [lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) repository without the `example_` prefix and the `.json` extension.

```bash
cargo lambda invoke --data-example apigw-request
```

For generic events, where you define the event data structure, you can create a JSON file with the data you want to test with. For example:

```json
{
  "command": "test"
}
```

Then, run `cargo lambda invoke --data-file ./data.json` to invoke the function with the data in `data.json`.

For HTTP events, you can also call the function directly with cURL or any other HTTP client. For example:

```bash
curl https://localhost:9000
```

Read more about running the local server in [the Cargo Lambda documentation for the `watch` command](https://www.cargo-lambda.info/commands/watch.html).
Read more about invoking the function in [the Cargo Lambda documentation for the `invoke` command](https://www.cargo-lambda.info/commands/invoke.html).

## Deploying

To deploy the project, run `cargo lambda deploy`. This will create an IAM role and a Lambda function in your AWS account.

Read more about deploying your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/deploy.html).

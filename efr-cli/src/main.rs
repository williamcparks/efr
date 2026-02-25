use std::process::ExitCode;

mod config;
mod error;
mod handler;
mod operations;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match handler::handler().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

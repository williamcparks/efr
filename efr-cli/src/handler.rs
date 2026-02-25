use strum::VariantArray;

use crate::{
    config::EfrConfig,
    error::CliError,
    operations::{Operations, authenticate_user, get_case, get_case_list},
};

pub async fn handler() -> Result<(), CliError> {
    let config = EfrConfig::try_new()?;
    let client = reqwest::ClientBuilder::new().build()?;

    let operation = inquire::Select::new(
        "What Operation Are You Looking To Preform?",
        Operations::VARIANTS.to_vec(),
    )
    .prompt()?;

    match operation {
        Operations::AuthenticateUser => {
            authenticate_user::handler(client, &config).await?;
        }
        Operations::GetCaseList => get_case_list::handler(client, &config).await?,
        Operations::GetCase => get_case::handler(client, &config).await?,
    }

    Ok(())
}

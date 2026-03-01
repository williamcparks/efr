use reqwest::Client;

use crate::{
    config::EfrConfig,
    operations::{error::OperationsError, get::get},
};

pub async fn location(client: Client, config: &EfrConfig) -> Result<(), OperationsError> {
    get(client, config, config.metadata.location_codes_url()).await
}

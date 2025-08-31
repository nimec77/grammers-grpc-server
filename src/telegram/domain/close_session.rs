use grammers_client::Client;
use log::info;

use crate::{
    app_config::TelegramConfig,
    telegram::{domain::utils::save_session, error::TelegramError},
};

#[allow(dead_code)]
pub(crate) async fn close_session(
    config: &TelegramConfig,
    client: &Client,
) -> Result<(), TelegramError> {
    info!("Closing session");

    save_session(client, config)?;

    client
        .sign_out()
        .await
        .map_err(|e| TelegramError::from(Box::new(e)))?;
    info!("Session closed");

    Ok(())
}

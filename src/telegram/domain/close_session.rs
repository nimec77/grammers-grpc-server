use grammers_client::Client;
use log::info;

use crate::{app_config::TelegramConfig, telegram::error::TelegramError};

#[allow(dead_code)]
pub(crate) async fn close_session(
    config: &TelegramConfig,
    client: &Client,
) -> Result<(), TelegramError> {
    info!("Closing session");
    client
        .session()
        .save_to_file(config.tg_session_file_path())
        .map_err(TelegramError::from)?;

    client
        .sign_out()
        .await
        .map_err(|e| TelegramError::from(Box::new(e)))?;
    info!("Session closed");

    Ok(())
}

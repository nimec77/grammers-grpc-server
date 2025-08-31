use grammers_client::Client;

use crate::{app_config::TelegramConfig, telegram::error::TelegramError};

pub(super) fn save_session(client: &Client, config: &TelegramConfig) -> Result<(), TelegramError> {
    client
        .session()
        .save_to_file(config.tg_session_file_path())
        .map_err(TelegramError::from)?;

    Ok(())
}

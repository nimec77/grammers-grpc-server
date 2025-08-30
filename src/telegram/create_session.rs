use anyhow::Context;
use grammers_client::{Client, Config as TgConfig, InitParams};
use grammers_session::Session;
use log::info;

use crate::app_config::TelegramConfig;

pub async fn create_session(config: &TelegramConfig) -> Result<Client, anyhow::Error> {
    info!("Connecting to Telegram");

    let session =
        Session::load_file_or_create(config.tg_session_file_path()).with_context(|| {
            format!(
                "Failed to load or create {}",
                config.tg_session_file_path().display()
            )
        })?;
    let tg_config = TgConfig {
        session,
        api_id: config.tg_id(),
        api_hash: config.tg_hash().into(),
        params: InitParams::default(),
    };

    let client = Client::connect(tg_config)
        .await
        .context("Failed to connect to Telegram")?;

    info!("Connected to Telegram");
    Ok(client)
}

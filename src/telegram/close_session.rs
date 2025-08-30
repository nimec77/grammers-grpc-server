use anyhow::Context;
use grammers_client::Client;
use log::info;

use crate::app_config::TelegramConfig;

pub async fn close_session(config: &TelegramConfig, client: &Client) -> Result<(), anyhow::Error> {
    info!("Closing session");
    client
        .session()
        .save_to_file(config.tg_session_file_path())
        .with_context(|| {
            format!(
                "Failed to save session to {}",
                config.tg_session_file_path().display()
            )
        })?;
        
    client.sign_out().await.context("Failed to sign out")?;
     info!("Session closed");
     
    Ok(())
}

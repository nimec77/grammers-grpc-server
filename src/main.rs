mod app_config;
mod logger;
mod telegram;

use anyhow::Context;
use log::info;

use crate::telegram::{TelegramRepository, grammers_repository::GrammersRepository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_logger();
    let app_config = app_config::AppConfig::load_config().context("Failed to load config")?;

    let mut grammers_repository = GrammersRepository::new(app_config.telegram());
    grammers_repository
        .create_session()
        .await
        .context("Failed to create session")?;

    let channels = grammers_repository
        .get_channels_list()
        .await
        .context("Failed to get channels list")?;

    info!("Channels total: {:?}", channels.len());

    grammers_repository
        .get_new_messages()
        .await
        .context("Failed to get new messages")?;

    Ok(())
}

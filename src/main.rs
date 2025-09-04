mod app_config;
mod logger;
mod telegram;

use anyhow::Context;
use log::info;
use tokio_stream::StreamExt;
use tokio::pin;

use crate::telegram::{
    TelegramRepository, grammers_repository::GrammersRepository,
    models::tg_messages_bus::TgMessagesBus,
};

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

    let tg_messages_bus = TgMessagesBus::new();

    let tg_messages_bus_read = tg_messages_bus.clone();
    tokio::spawn(async move {
        if let Err(e) = grammers_repository
            .get_new_messages(&tg_messages_bus_read)
            .await
        {
            panic!("Failed to get new messages: {:?}", e);
        }
    });

    let tg_sub = tg_messages_bus.subscribe();
    pin!(tg_sub);
    while let Some(message) = tg_sub.next().await {
        info!("New message: {:?}", message);
    }

    Ok(())
}

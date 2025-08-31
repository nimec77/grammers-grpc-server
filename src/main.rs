mod app_config;
mod logger;
mod telegram;

use anyhow::Context;
use log::info;

use crate::telegram::{TelegramRepository, grammers_repository::GrammersRepository};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    logger::init_logger();
    let app_config = app_config::AppConfig::load_config().context("Failed to load config")?;

    let mut grammers_repository = GrammersRepository::new(app_config.telegram());
    grammers_repository
        .create_session()
        .await
        .context("Failed to create session")?;

    // let client = create_session(app_config.telegram())
    //     .await
    //     .expect("Failed to create session");
    // let channels = get_channels_list(&client)
    //     .await
    //     .expect("Failed to get dialogs list");

    // info!("Channels total: {:?}", channels.len());

    // get_new_messages(&client)
    //     .await
    //     .expect("Failed to get new messages");

    Ok(())
}

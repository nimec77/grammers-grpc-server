mod app_config;
mod logger;
mod telegram;

use log::info;

use crate::telegram::{
    create_session::create_session, get_channels_list::get_channels_list,
    get_new_messages::get_new_messages,
};

#[tokio::main]
async fn main() {
    logger::init_logger();
    let app_config = app_config::AppConfig::load_config().expect("Failed to load config");

    let client = create_session(app_config.telegram())
        .await
        .expect("Failed to create session");
    let channels = get_channels_list(&client)
        .await
        .expect("Failed to get dialogs list");

    info!("Channels total: {:?}", channels.len());

    get_new_messages(&client)
        .await
        .expect("Failed to get new messages");
}

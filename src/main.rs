mod app_config;
mod logger;
mod telegram;

use crate::telegram::{create_session::create_session, get_dialogs_list::get_dialogs_list};

#[tokio::main]
async fn main() {
    logger::init_logger();
    let app_config = app_config::AppConfig::load_config().expect("Failed to load config");

    let client = create_session(app_config.telegram())
        .await
        .expect("Failed to create session");
    get_dialogs_list(&client)
        .await
        .expect("Failed to get dialogs list");
}

use grammers_client::{Client, Update, types::Chat};
use log::{debug, info};

use crate::telegram::{
    error::TelegramError,
    models::{tg_messages_bus::TgMessagesBus, ts_message::TgMessage},
};

pub async fn get_new_messages(
    client: &Client,
    tg_messages_bus: &TgMessagesBus,
) -> Result<(), TelegramError> {
    info!("Getting new messages");
    loop {
        let update = client
            .next_update()
            .await
            .map_err(|e| TelegramError::from(Box::new(e)))?;

        if let Update::NewMessage(message) = update
            && !message.text().is_empty()
        {
            let chat = message.chat();
            if let Chat::Channel(channel) = chat {
                let tg_message = TgMessage::new(channel.id(), channel.title(), message.text());
                debug!("New message: {:?}", tg_message);
                tg_messages_bus
                    .publish(tg_message)
                    .map_err(TelegramError::from)?;
            }
        }
    }
}

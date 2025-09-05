use grammers_client::{Client, Update, types::Chat};
use log::{debug, info};
use tokio_util::sync::CancellationToken;

use crate::telegram::{
    error::TelegramError,
    models::{tg_messages_bus::TgMessagesBus, ts_message::TgMessage},
};

pub async fn get_new_messages(
    client: &Client,
    tg_messages_bus: &TgMessagesBus,
    token: CancellationToken,
) -> Result<(), TelegramError> {
    info!("Getting new messages");

    // An owned future that resolves when `token` is cancelled.
    let cancelled = token.cancelled_owned();
    tokio::pin!(cancelled);

    loop {
        tokio::select! {
            // (Optional) prefer cancellation if both branches are ready at once
            biased;

            _ = &mut cancelled => {
                info!("Cancellation requested — stopping Telegram listener");
                return Ok(());
            }

            res = client.next_update() => {
                let update = res.map_err(|e| TelegramError::from(Box::new(e)))?;

                if let Update::NewMessage(message) = update
                    && !message.text().is_empty()
                    && let Chat::Channel(channel) = message.chat() {
                        let tg_message = TgMessage::new(channel.id(), channel.title(), message.text());
                        debug!("New message: {:?}", tg_message);
                        tg_messages_bus.publish(tg_message).map_err(TelegramError::from)?;
                    }
            }
        }
    }
}

use grammers_client::{
    Client,
    types::{Channel, Chat},
};
use log::{debug, info};

use crate::telegram::error::TelegramError;

pub(crate) async fn get_channels_list(client: &Client) -> Result<Vec<Channel>, TelegramError> {
    info!("Getting channels list");
    let mut dialogs = client.iter_dialogs();

    let total = dialogs
        .total()
        .await
        .map_err(|e| TelegramError::from(Box::new(e)))?;
    debug!("Dialogs total: {:?}", total);

    let mut channels = vec![];
    while let Some(dialog) = dialogs
        .next()
        .await
        .map_err(|e| TelegramError::from(Box::new(e)))?
    {
        let chat = dialog.chat();
        if let Chat::Channel(channel) = chat {
            debug!("- {: >10} {}", channel.id(), channel.title());
            channels.push(channel.clone());
        }
    }

    info!("Channels list received");
    Ok(channels)
}

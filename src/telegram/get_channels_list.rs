use anyhow::Context;
use grammers_client::{types::{Channel, Chat}, Client};
use log::{debug, info};

pub async fn get_channels_list(client: &Client) -> Result<Vec<Channel>, anyhow::Error> {
    info!("Getting channels list");
    let mut dialogs = client.iter_dialogs();

    let total = dialogs
        .total()
        .await
        .context("Failed to get dialogs total")?;
    debug!("Dialogs total: {:?}", total);

    let mut channels = vec![];
    while let Some(dialog) = dialogs.next().await.context("Failed to get next dialog")? {
        let chat = dialog.chat();
        if let Chat::Channel(channel) = chat {
            debug!("- {: >10} {}", channel.id(), channel.title());
            channels.push(channel.clone());
        }
    }

    info!("Channels list received");
    Ok(channels)
}

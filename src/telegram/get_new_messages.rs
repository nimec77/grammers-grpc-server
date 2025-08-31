use anyhow::Context;
use grammers_client::{Client, Update, types::Chat};
use log::{debug, info};

pub async fn get_new_messages(client: &Client) -> Result<(), anyhow::Error> {
    info!("Getting new messages");
    let mut count = 0;
    while count < 1 {
        let update = client
            .next_update()
            .await
            .context("Failed to get next update")?;
        if let Update::NewMessage(message) = update {
            let chat = message.chat();
            if let Chat::Channel(channel) = chat {
                debug!(
                    "Message from channel {}: {}",
                    channel.title(),
                    message.text()
                );
                count += 1;
            }
        }
    }
    info!("New messages received");
    Ok(())
}

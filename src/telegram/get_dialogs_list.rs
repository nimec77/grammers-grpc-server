use anyhow::Context;
use grammers_client::Client;
use log::info;

pub async fn get_dialogs_list(client: &Client) -> Result<(), anyhow::Error> {
    let mut dialogs = client.iter_dialogs();

    let total = dialogs.total().await.context("Failed to get dialogs total")?;
    info!("Dialogs total: {:?}", total);

    while let Some(dialog) = dialogs.next().await.context("Failed to get next dialog")? {
        let chat = dialog.chat();
        info!("- {: >10} {}", chat.id(), chat.name().unwrap_or_default());
    }
    
    Ok(())
}

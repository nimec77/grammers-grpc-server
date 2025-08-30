use anyhow::Context;
use grammers_client::Client;
use log::info;

pub async fn get_dialogs_list(client: &Client) -> Result<(), anyhow::Error> {
    let mut dialogs = client.iter_dialogs();

    let total = dialogs.total().await.context("Failed to get dialogs total")?;
    info!("Dialogs total: {:?}", total);


    Ok(())
}

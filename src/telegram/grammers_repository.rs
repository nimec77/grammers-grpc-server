use async_trait::async_trait;
use grammers_client::{Client, types::Channel};

use crate::{
    app_config::TelegramConfig,
    telegram::{
        TelegramRepository,
        domain::{close_session, create_session, get_channels_list},
        error::TelegramError,
    },
};

pub struct GrammersRepository {
    config: TelegramConfig,
    client: Option<Client>,
}

#[async_trait]
impl TelegramRepository for GrammersRepository {
    fn new(config: &TelegramConfig) -> Self {
        Self {
            config: config.clone(),
            client: None,
        }
    }

    async fn create_session(&mut self) -> Result<(), TelegramError> {
        let client = create_session::create_session(&self.config).await?;
        self.client = Some(client);

        Ok(())
    }

    async fn close_session(&self) -> Result<(), TelegramError> {
        if let Some(client) = &self.client {
            close_session::close_session(&self.config, client).await
        } else {
            Err(TelegramError::ClientNotFound)
        }
    }

    async fn get_channels_list(&self) -> Result<Vec<Channel>, TelegramError> {
        if let Some(client) = &self.client {
            get_channels_list::get_channels_list(client).await
        } else {
            Err(TelegramError::ClientNotFound)
        }
    }
}

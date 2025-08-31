use async_trait::async_trait;
use grammers_client::types::Channel;

use crate::{app_config::TelegramConfig, telegram::error::TelegramError};

pub mod domain;

pub mod error;

pub mod grammers_repository;

#[async_trait]
pub trait TelegramRepository {
    fn new(config: &TelegramConfig) -> Self;
    async fn create_session(&mut self) -> Result<(), TelegramError>;

    #[allow(dead_code)]
    async fn close_session(&mut self) -> Result<(), TelegramError>;

    async fn get_channels_list(&self) -> Result<Vec<Channel>, TelegramError>;

    async fn get_new_messages(&self) -> Result<(), TelegramError>;
}

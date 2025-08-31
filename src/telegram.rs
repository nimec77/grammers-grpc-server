use async_trait::async_trait;
use grammers_client::InvocationError;

use crate::app_config::TelegramConfig;

pub mod create_session;

pub mod close_session;

pub mod get_channels_list;

pub mod get_new_messages;

#[async_trait]
pub trait TelegramClient {
    fn new(config: &TelegramConfig) -> Self;
    // fn get_channels_list(&self) -> Result<Vec<Channel>, anyhow::Error>;
    // fn get_new_messages(&self) -> Result<(), anyhow::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum TelegramError {
    // ----- General RPC invocation failures (network, decode, dropped, or server RPC error) -----
    #[error("telegram RPC invocation failed: {0}")]
    Invoke(#[from] InvocationError),
}

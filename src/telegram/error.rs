use grammers_client::{InvocationError, SignInError, client::bots::AuthorizationError};
use tokio::sync::broadcast;

use crate::telegram::models::ts_message::TgMessage;

#[derive(Debug, thiserror::Error)]
pub enum TelegramError {
    // ----- General RPC invocation failures (network, decode, dropped, or server RPC error) -----
    #[error("telegram RPC invocation failed: {0}")]
    Invoke(#[from] Box<InvocationError>),

    // ----- Session/config/glue around grammers -----
    #[error("session I/O: {0}")]
    IoError(#[from] std::io::Error),

    // ----- Authorization errors -----
    #[error("authorization failed: {0}")]
    AuthError(#[from] Box<AuthorizationError>),

    #[error("sign in failed: {0}")]
    SignError(#[from] Box<SignInError>),

    // ----- User input errors -----
    #[error("user input error: {0}")]
    InputError(#[from] dialoguer::Error),

    #[error("client not found")]
    ClientNotFound,

    #[error("broadcast error: {0}")]
    BroadcastError(#[from] broadcast::error::SendError<TgMessage>),
}

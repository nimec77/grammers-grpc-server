
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("telegram error: {0}")]
    Telegram(TelegramError),
}

#[derive(thiserror::Error, Debug)]
pub enum TelegramError {
    // Client(grammers_client::error::Error),
    // Session(grammers_session::error::Error),
    // SignIn(grammers_client::SignInError),
    // SignOut(grammers_client::SignOutError),
    // GetChannelsList(grammers_client::error::Error),
    // GetNewMessages(grammers_client::error::Error),
}

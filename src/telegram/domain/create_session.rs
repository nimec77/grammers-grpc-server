use dialoguer::{Input, Password};
use grammers_client::{Client, Config as TgConfig, InitParams, SignInError};
use grammers_session::Session;
use log::info;

use crate::{app_config::TelegramConfig, telegram::error::TelegramError};

pub(crate) async fn create_session(config: &TelegramConfig) -> Result<Client, TelegramError> {
    info!("Connecting to Telegram");

    let session_file_path = config.tg_session_file_path();
    let session = Session::load_file_or_create(session_file_path)
        .map_err(TelegramError::from)?;
    let tg_config = TgConfig {
        session,
        api_id: config.tg_id(),
        api_hash: config.tg_hash().into(),
        params: InitParams::default(),
    };

    let client = Client::connect(tg_config)
        .await
        .map_err(|e| TelegramError::from(Box::new(e)))?;

    let is_authorized = client
        .is_authorized()
        .await
        .map_err(|e| TelegramError::from(Box::new(e)))?;
    if is_authorized {
        info!("Already authorized");
        return Ok(client);
    }

    let token = client
        .request_login_code(config.tg_phone())
        .await
        .map_err(|e| TelegramError::from(Box::new(e)))?;

    let code: String = Input::new()
        .with_prompt("Enter the login code you received: ")
        .interact_text()
        .map_err(TelegramError::from)?;

    let signed_in = client.sign_in(&token, &code).await;
    match signed_in {
        Ok(_) => {
            info!("Signed in with code successfully");
            return Ok(client);
        }
        Err(SignInError::PasswordRequired(pw_token)) => {
            let hint = pw_token.hint().unwrap_or("none");
            let password = Password::new()
                .with_prompt(format!("Enter the password for {hint}: "))
                .interact()
                .map_err(TelegramError::from)?;

            client
                .check_password(pw_token, password.as_bytes())
                .await
                .map_err(|e| TelegramError::from(Box::new(e)))?;

            info!("Signed in with password successfully");
        }
        Err(e) => return Err(TelegramError::from(Box::new(e))),
    }

    client
        .session()
        .save_to_file(session_file_path)
        .map_err(TelegramError::from)?;

    Ok(client)
}

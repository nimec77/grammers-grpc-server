use std::path::{Path, PathBuf};

use secrecy::{ExposeSecret, SecretBox, SecretString};

pub const TG_ID_KEY: &str = "TG_ID";
pub const TG_HASH_KEY: &str = "TG_HASH";
pub const TG_PHONE_KEY: &str = "TG_PHONE";
pub const TG_SESSION_FILE_PATH_KEY: &str = "TG_SESSION_FILE_PATH";

pub struct AppConfig {
    telegram: TelegramConfig,
}

impl AppConfig {
    pub fn telegram(&self) -> &TelegramConfig {
        &self.telegram
    }

    pub fn load_config() -> Result<Self, anyhow::Error> {
        let telegram = TelegramConfig::load_config()?;

        Ok(Self { telegram })
    }
}

#[derive(Debug, Clone)]
pub struct TelegramConfig {
    tg_id: SecretBox<i32>,
    tg_hash: SecretString,
    tg_phone: SecretString,
    tg_session_file_path: PathBuf,
}

impl TelegramConfig {
    pub fn tg_id(&self) -> i32 {
        *self.tg_id.expose_secret()
    }

    pub fn tg_hash(&self) -> &str {
        self.tg_hash.expose_secret()
    }

    pub fn tg_phone(&self) -> &str {
        self.tg_phone.expose_secret()
    }

    pub fn tg_session_file_path(&self) -> &Path {
        self.tg_session_file_path.as_path()
    }

    pub fn load_config() -> Result<Self, anyhow::Error> {
        log::debug!("Loading config");

        let must_tg_id = format!("{TG_ID_KEY} must be set");
        let must_tg_id_integer = format!("{TG_ID_KEY} must be a valid integer");
        let tg_id = dotenvy::var(TG_ID_KEY)
            .expect(&must_tg_id)
            .parse::<i32>()
            .expect(&must_tg_id_integer);

        let must_tg_hash = format!("{TG_HASH_KEY} must be set");
        let tg_hash = dotenvy::var(TG_HASH_KEY).expect(&must_tg_hash);
        let must_tg_phone = format!("{TG_PHONE_KEY} must be set");
        let tg_phone = dotenvy::var(TG_PHONE_KEY).expect(&must_tg_phone);
        let must_tg_session_file_path = format!("{TG_SESSION_FILE_PATH_KEY} must be set");
        let tg_session_file_path = dotenvy::var(TG_SESSION_FILE_PATH_KEY)
            .expect(&must_tg_session_file_path)
            .into();

        let config = Self {
            tg_id: SecretBox::new(Box::new(tg_id)),
            tg_hash: SecretString::new(tg_hash.into_boxed_str()),
            tg_phone: SecretString::new(tg_phone.into_boxed_str()),
            tg_session_file_path,
        };
        log::debug!("Config loaded: {:?}", config);

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{remove_var, set_var};

    const TG_ID: &str = "1234567890";
    const TG_HASH: &str = "aslkdfjsdklfjsdklfjklsdjflksjdf";
    const TG_PHONE: &str = "+79111234567";
    const TG_SESSION_FILE_PATH: &str = "/tmp/tg_session.json";

    fn setup_env() {
        unsafe {
            set_var(TG_ID_KEY, TG_ID);
            set_var(TG_HASH_KEY, TG_HASH);
            set_var(TG_PHONE_KEY, TG_PHONE);
            set_var(TG_SESSION_FILE_PATH_KEY, TG_SESSION_FILE_PATH);
        };
    }

    #[allow(dead_code)]
    fn teardown_env() {
        unsafe {
            remove_var(TG_ID_KEY);
            remove_var(TG_HASH_KEY);
            remove_var(TG_PHONE_KEY);
            remove_var(TG_SESSION_FILE_PATH_KEY);
        };
    }

    #[test]
    fn test_telegram_load_config() {
        setup_env();

        let config = TelegramConfig::load_config().expect("Failed to load config");

        assert_eq!(config.tg_id(), TG_ID.parse::<i32>().unwrap());
        assert_eq!(config.tg_hash(), TG_HASH);
        assert_eq!(config.tg_phone(), TG_PHONE);
        assert_eq!(
            config.tg_session_file_path().to_string_lossy(),
            TG_SESSION_FILE_PATH
        );
    }

    #[test]
    fn test_app_load_config() {
        setup_env();

        let config = AppConfig::load_config().expect("Failed to load config");
        let telegram_config = config.telegram();
        assert_eq!(telegram_config.tg_id(), TG_ID.parse::<i32>().unwrap());
        assert_eq!(telegram_config.tg_hash(), TG_HASH);
        assert_eq!(telegram_config.tg_phone(), TG_PHONE);
        assert_eq!(
            telegram_config.tg_session_file_path().to_string_lossy(),
            TG_SESSION_FILE_PATH
        );
    }
}

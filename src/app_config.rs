use std::path::{Path, PathBuf};

use secrecy::{ExposeSecret, SecretBox, SecretString};

#[derive(Debug, Clone)]
pub struct AppConfig {
    tg_id: SecretBox<i32>,
    tg_hash: SecretString,
    tg_phone: SecretString,
    tg_session_file_path: PathBuf,
}

impl AppConfig {
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
        let tg_id = dotenvy::var("TG_ID")
            .expect("TG_ID must be set")
            .parse::<i32>()
            .expect("TG_ID must be a valid integer");
        let tg_hash = dotenvy::var("TG_HASH").expect("TG_HASH must be set");
        let tg_phone = dotenvy::var("TG_PHONE").expect("TG_PHONE must be set");
        let tg_session_file_path = dotenvy::var("TG_SESSION_FILE_PATH")
            .expect("TG_SESSION_FILE_PATH must be set")
            .into();


        let config = Self {
            tg_id: SecretBox::new(Box::new(tg_id)),
            tg_hash: SecretString::new(tg_hash.into_boxed_str()),
            tg_phone: SecretString::new(tg_phone.into_boxed_str()),
            tg_session_file_path,
        };
        log::info!("Config loaded: {:?}", config);

        Ok(config)
    }
}

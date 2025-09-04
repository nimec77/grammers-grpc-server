#[derive(Clone, Debug)]
pub struct TgMessage {
    chat_id: i64,
    chat_title: String,
    message: String,
}

impl TgMessage {
    pub fn new(chat_id: i64, chat_title: &str, message: &str) -> Self {
        Self {
            chat_id,
            chat_title: chat_title.to_string(),
            message: message.to_string(),
        }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn chat_title(&self) -> &str {
        &self.chat_title.as_ref()
    }

    pub fn message(&self) -> &str {
        &self.message.as_ref()
    }
}

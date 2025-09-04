use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};

use crate::telegram::models::ts_message::TgMessage;

#[derive(Clone)]
pub struct TgMessagesBus {
    tx: broadcast::Sender<TgMessage>,
}

impl TgMessagesBus {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1);

        Self { tx }
    }

    pub fn publish(&self, message: TgMessage) -> Result<(), broadcast::error::SendError<TgMessage>> {
        self.tx.send(message)?;

        Ok(())
    }

    pub fn subscribe(&self) -> impl Stream<Item = TgMessage> {
        BroadcastStream::new(self.tx.subscribe()).then(|res| async move {
            res.ok()
        }).filter_map(|opt| opt)
    }
}

impl Default for TgMessagesBus {
    fn default() -> Self {
        Self::new()
    }
}

use super::Trigger;
use anyhow::Result;
use log::error;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

#[derive(Debug, Serialize, Deserialize)]
pub struct AfterAppvisLaunched {}

impl Trigger for AfterAppvisLaunched {
    async fn observe(&self, tx: Sender<Result<()>>) {
        if let Err(e) = tx.send(Ok(())).await {
            error!("failed to send notify: {}", e);
        }
    }

    fn channel_buffer_size(&self) -> usize {
        1
    }
}

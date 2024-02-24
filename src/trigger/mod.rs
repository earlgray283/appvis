use self::{network::WifiConnected, system::AfterAppvisLaunched};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

pub mod network;
pub mod system;

pub trait Trigger {
    async fn observe(&self, tx: Sender<Result<()>>);
    fn channel_buffer_size(&self) -> usize;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "properties")]
pub enum TriggerKind {
    WifiConnected(WifiConnected),
    AfterAppvisLaunched(AfterAppvisLaunched),
}

impl Trigger for TriggerKind {
    async fn observe(&self, tx: Sender<Result<()>>) {
        match self {
            Self::WifiConnected(t) => t.observe(tx).await,
            Self::AfterAppvisLaunched(t) => t.observe(tx).await,
        }
    }

    fn channel_buffer_size(&self) -> usize {
        match self {
            Self::WifiConnected(t) => t.channel_buffer_size(),
            Self::AfterAppvisLaunched(t) => t.channel_buffer_size(),
        }
    }
}

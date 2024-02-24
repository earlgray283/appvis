use self::{network::WifiConnected, system::AfterLaunchSchedulerLaunched};
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
    AfterLaunchSchedulerLaunched(AfterLaunchSchedulerLaunched),
}

impl Trigger for TriggerKind {
    async fn observe(&self, tx: Sender<Result<()>>) {
        match self {
            Self::WifiConnected(t) => t.observe(tx).await,
            Self::AfterLaunchSchedulerLaunched(t) => t.observe(tx).await,
        }
    }

    fn channel_buffer_size(&self) -> usize {
        match self {
            Self::WifiConnected(t) => t.channel_buffer_size(),
            Self::AfterLaunchSchedulerLaunched(t) => t.channel_buffer_size(),
        }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncedItemProgress {
    #[serde(rename(deserialize = "Progress"), skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(rename(deserialize = "Status"))]
    pub status: Status,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "Converting")]
    Converting,
    #[serde(rename = "ReadyToTransfer")]
    ReadyToTransfer,
    #[serde(rename = "Transferring")]
    Transferring,
    #[serde(rename = "Synced")]
    Synced,
    #[serde(rename = "Failed")]
    Failed,
}


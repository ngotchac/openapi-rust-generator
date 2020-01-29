use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncDataRequest {
    #[serde(rename(deserialize = "LocalItemIds"))]
    pub local_item_ids: Vec<String>,
    #[serde(rename(deserialize = "TargetId"))]
    pub target_id: String,
}



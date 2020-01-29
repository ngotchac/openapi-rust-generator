use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncDataResponse {
    #[serde(rename(deserialize = "ItemIdsToRemove"))]
    pub item_ids_to_remove: Vec<String>,
}



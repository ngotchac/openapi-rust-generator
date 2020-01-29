use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResultSyncSyncJob {
    #[serde(rename(deserialize = "Items"))]
    pub items: Vec<Box<crate::models::SyncSyncJob>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
}



use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncJobCreationResult {
    #[serde(rename(deserialize = "Job"))]
    pub job: Box<crate::models::SyncSyncJob>,
    #[serde(rename(deserialize = "JobItems"))]
    pub job_items: Vec<Box<crate::models::SyncModelSyncJobItem>>,
}



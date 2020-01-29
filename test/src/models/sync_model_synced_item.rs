use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncedItem {
    #[serde(rename(deserialize = "ServerId"))]
    pub server_id: String,
    #[serde(rename(deserialize = "SyncJobId"))]
    pub sync_job_id: i64,
    #[serde(rename(deserialize = "SyncJobName"))]
    pub sync_job_name: String,
    #[serde(rename(deserialize = "SyncJobDateCreated"))]
    pub sync_job_date_created: String,
    #[serde(rename(deserialize = "SyncJobItemId"))]
    pub sync_job_item_id: i64,
    #[serde(rename(deserialize = "OriginalFileName"))]
    pub original_file_name: String,
    #[serde(rename(deserialize = "Item"))]
    pub item: Box<crate::models::BaseItemDto>,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "AdditionalFiles"))]
    pub additional_files: Vec<Box<crate::models::SyncModelItemFileInfo>>,
}



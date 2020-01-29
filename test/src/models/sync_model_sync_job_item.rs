use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncJobItem {
    #[serde(rename(deserialize = "Id"))]
    pub id: i64,
    #[serde(rename(deserialize = "JobId"))]
    pub job_id: i64,
    #[serde(rename(deserialize = "ItemId"))]
    pub item_id: i64,
    #[serde(rename(deserialize = "ItemName"))]
    pub item_name: String,
    #[serde(rename(deserialize = "MediaSourceId"))]
    pub media_source_id: String,
    #[serde(rename(deserialize = "MediaSource"))]
    pub media_source: Box<crate::models::MediaSourceInfo>,
    #[serde(rename(deserialize = "TargetId"))]
    pub target_id: String,
    #[serde(rename(deserialize = "OutputPath"))]
    pub output_path: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: Status,
    #[serde(rename(deserialize = "Progress"), skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(rename(deserialize = "DateCreated"))]
    pub date_created: String,
    #[serde(rename(deserialize = "PrimaryImageItemId"))]
    pub primary_image_item_id: i64,
    #[serde(rename(deserialize = "PrimaryImageTag"))]
    pub primary_image_tag: String,
    #[serde(rename(deserialize = "TemporaryPath"))]
    pub temporary_path: String,
    #[serde(rename(deserialize = "AdditionalFiles"))]
    pub additional_files: Vec<Box<crate::models::SyncModelItemFileInfo>>,
    #[serde(rename(deserialize = "ItemDateModifiedTicks"))]
    pub item_date_modified_ticks: i64,
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


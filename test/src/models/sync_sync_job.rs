use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncSyncJob {
    #[serde(rename(deserialize = "Id"))]
    pub id: i64,
    #[serde(rename(deserialize = "TargetId"))]
    pub target_id: String,
    #[serde(rename(deserialize = "TargetName"))]
    pub target_name: String,
    #[serde(rename(deserialize = "Quality"))]
    pub quality: String,
    #[serde(rename(deserialize = "Bitrate"), skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename(deserialize = "Profile"))]
    pub profile: String,
    #[serde(rename(deserialize = "Category"))]
    pub category: Category,
    #[serde(rename(deserialize = "ParentId"))]
    pub parent_id: i64,
    #[serde(rename(deserialize = "Progress"))]
    pub progress: f64,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: Status,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: i64,
    #[serde(rename(deserialize = "UnwatchedOnly"))]
    pub unwatched_only: bool,
    #[serde(rename(deserialize = "SyncNewContent"))]
    pub sync_new_content: bool,
    #[serde(rename(deserialize = "ItemLimit"), skip_serializing_if = "Option::is_none")]
    pub item_limit: Option<i32>,
    #[serde(rename(deserialize = "RequestedItemIds"))]
    pub requested_item_ids: Vec<i64>,
    #[serde(rename(deserialize = "DateCreated"))]
    pub date_created: String,
    #[serde(rename(deserialize = "DateLastModified"))]
    pub date_last_modified: String,
    #[serde(rename(deserialize = "ItemCount"))]
    pub item_count: i32,
    #[serde(rename(deserialize = "ParentName"))]
    pub parent_name: String,
    #[serde(rename(deserialize = "PrimaryImageItemId"))]
    pub primary_image_item_id: String,
    #[serde(rename(deserialize = "PrimaryImageTag"))]
    pub primary_image_tag: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "Latest")]
    Latest,
    #[serde(rename = "NextUp")]
    NextUp,
    #[serde(rename = "Resume")]
    Resume,
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
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "CompletedWithError")]
    CompletedWithError,
    #[serde(rename = "Failed")]
    Failed,
}


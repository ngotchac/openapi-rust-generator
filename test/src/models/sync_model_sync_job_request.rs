use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncJobRequest {
    #[serde(rename(deserialize = "TargetId"))]
    pub target_id: String,
    #[serde(rename(deserialize = "ItemIds"))]
    pub item_ids: Vec<String>,
    #[serde(rename(deserialize = "Category"))]
    pub category: Category,
    #[serde(rename(deserialize = "ParentId"))]
    pub parent_id: String,
    #[serde(rename(deserialize = "Quality"))]
    pub quality: String,
    #[serde(rename(deserialize = "Profile"))]
    pub profile: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "UnwatchedOnly"))]
    pub unwatched_only: bool,
    #[serde(rename(deserialize = "SyncNewContent"))]
    pub sync_new_content: bool,
    #[serde(rename(deserialize = "ItemLimit"), skip_serializing_if = "Option::is_none")]
    pub item_limit: Option<i32>,
    #[serde(rename(deserialize = "Bitrate"), skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
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

